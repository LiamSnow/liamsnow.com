use crate::{CONFIG, typst};
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::mem;
use std::ops::Bound;
use std::path::PathBuf;
use tokio::fs;
use tokio::task::JoinSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMeta {
    pub title: String,
    #[serde(default)]
    pub query: Vec<String>,
    /// injected indexer
    #[serde(default)]
    pub url: String,
    #[serde(flatten)]
    pub extra: Value,
}

pub struct Task {
    pub path: PathBuf,
    pub ty: TaskType,
}

pub enum TaskType {
    Page {
        meta: PageMeta,
        query_result: String,
    },
    File,
}

impl Task {
    fn file(path: PathBuf) -> Self {
        Self {
            path,
            ty: TaskType::File,
        }
    }
}

/// Index all files in `content`, get metadata, & resolve queries
pub async fn index() -> Result<BTreeMap<String, Task>> {
    let content_dir = &CONFIG.get().unwrap().content_dir;
    let (mut num_pages, mut num_files) = (0, 0);
    let mut typst_tasks = JoinSet::new();
    let mut stack = vec![fs::read_dir(content_dir).await?];
    let mut index = BTreeMap::new();

    while let Some(iter) = stack.last_mut() {
        let Some(entry) = iter.next_entry().await? else {
            stack.pop();
            continue;
        };

        let file_type = entry.file_type().await?;

        if file_type.is_dir() {
            stack.push(fs::read_dir(entry.path()).await?);
            continue;
        }

        if file_type.is_file() {
            let path = entry.path();
            let rel = path
                .strip_prefix(content_dir)?
                .to_str()
                .context("non-UTF-8 path")?;

            if is_hidden(rel) {
                continue;
            }

            let url = make_url(rel);

            if rel.ends_with(".typ") {
                num_pages += 1;
                typst_tasks.spawn(process_typst(path, url));
            } else {
                num_files += 1;
                index.insert(url, Task::file(path));
            }
        }
    }

    println!("  Found {num_pages} pages, {num_files} files");

    println!("  Extracing metadata...");
    let mut tbe = FxHashMap::default();
    while let Some(result) = typst_tasks.join_next().await {
        let (url, task, query) = result??;
        if !query.is_empty() {
            tbe.insert(url.clone(), query);
        }
        index.insert(url, task);
    }

    println!("  Evaluating queries...");
    for (url, query) in tbe {
        let outer: Vec<Vec<&PageMeta>> = query
            .iter()
            .map(|part| {
                let mut end = part.clone();
                if let Some(last) = end.as_bytes().last().copied() {
                    end.pop();
                    end.push((last + 1) as char);
                }
                index
                    .range::<str, _>((
                        Bound::Included(part.as_str()),
                        Bound::Excluded(end.as_str()),
                    ))
                    .filter_map(|(_, t)| match &t.ty {
                        TaskType::Page { meta, .. } => Some(meta),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        let res = serde_json::to_string(&outer)?;
        if let TaskType::Page { query_result, .. } = &mut index.get_mut(&url).unwrap().ty {
            *query_result = res;
        }
    }

    Ok(index)
}

async fn process_typst(path: PathBuf, url: String) -> Result<(String, Task, Vec<String>)> {
    let mut meta = typst::get_metadata(&path)
        .await
        .with_context(|| format!("parsing {}", path.display()))?;
    meta.url = url.clone();
    let query = mem::take(&mut meta.query);
    Ok((
        url,
        Task {
            path,
            ty: TaskType::Page {
                meta,
                query_result: "[]".to_string(),
            },
        },
        query,
    ))
}

/// `index.typ`      → `/`
/// `cat/index.typ`  → `/cat`
/// `cat/dog.typ`    → `/cat/dog`
/// `cat/robots.txt` → `/cat/robots.txt`
/// `style.scss`     → `/style.css`
fn make_url(rel: &str) -> String {
    if let Some(stem) = rel.strip_suffix(".typ") {
        if stem == "index" {
            "/".to_string()
        } else {
            format!("/{}", stem.strip_suffix("/index").unwrap_or(stem))
        }
    } else if let Some(stem) = rel
        .strip_suffix(".scss")
        .or_else(|| rel.strip_suffix(".sass"))
    {
        format!("/{stem}.css")
    } else {
        format!("/{rel}")
    }
}

fn is_hidden(rel: &str) -> bool {
    rel.split('/').any(|seg| seg.starts_with('_'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_url() {
        assert_eq!(make_url("index.typ"), "");
        assert_eq!(make_url("cat/index.typ"), "cat");
        assert_eq!(make_url("cat/dog.typ"), "cat/dog");
        assert_eq!(make_url("style.scss"), "style.css");
        assert_eq!(make_url("theme.sass"), "theme.css");
        assert_eq!(make_url("robots.txt"), "robots.txt");
        assert_eq!(make_url("img/photo.png"), "img/photo.png");
    }

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden("_drafts/post.typ"));
        assert!(is_hidden("blog/_hidden/post.typ"));
        assert!(!is_hidden("blog/post.typ"));
        assert!(!is_hidden("underscored_name/post.typ"));
    }
}
