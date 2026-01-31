use crate::{CONTENT_DIR, typst};
use anyhow::{Result, bail};
use rustc_hash::{FxBuildHasher, FxHashMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::task::JoinSet;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMeta {
    pub title: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageMetaWithUrl {
    pub url: String,
    pub title: String,
    #[serde(flatten)]
    pub extra: Value,
}

impl PageMetaWithUrl {
    pub fn from_meta(url: String, meta: &PageMeta) -> Self {
        Self {
            url,
            title: meta.title.clone(),
            extra: meta.extra.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageTask {
    pub file_path: PathBuf,
    pub url: String,
    pub meta: Option<PageMeta>,
    pub query_prefixes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileTask {
    pub file_path: PathBuf,
    pub url: String,
}

pub struct Index {
    pub pages: Vec<PageTask>,
    pub files: Vec<FileTask>,
    pub resolved_queries: FxHashMap<String, Vec<Vec<PageMetaWithUrl>>>,
}

/// Index all files in `content`, resolve queries, & get metadata
pub async fn index() -> Result<Index> {
    let content_dir = Path::new(CONTENT_DIR);

    println!("  Discovering files...");
    let (page_tasks, file_tasks) = discover_files(content_dir)?;
    println!(
        "  Found {} pages, {} files",
        page_tasks.len(),
        file_tasks.len()
    );

    println!("  Extracting metadata...");
    let page_tasks = extract_all_metadata(page_tasks).await?;

    println!("  Building index...");
    let page_index = build_index(&page_tasks);

    println!("  Resolving queries...");
    let resolved_queries = resolve_queries(&page_tasks, &page_index);

    Ok(Index {
        pages: page_tasks,
        files: file_tasks,
        resolved_queries,
    })
}

/// Walk through `content` dir, finding all pages and files
fn discover_files(content_dir: &Path) -> Result<(Vec<PageTask>, Vec<FileTask>)> {
    let mut pages = Vec::new();
    let mut files = Vec::new();

    for entry in WalkDir::new(content_dir) {
        let Ok(entry) = entry else {
            continue;
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let relative = path.strip_prefix(content_dir)?;

        if is_hidden_path(relative) {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase);

        match ext.as_deref() {
            Some("typ") => {
                let url = file_path_to_url(content_dir, path);
                pages.push(PageTask {
                    file_path: path.to_path_buf(),
                    url,
                    meta: None,
                    query_prefixes: vec![],
                });
            }
            Some("scss" | "sass") => {
                let url = format!("/{}", relative.with_extension("css").display());
                files.push(FileTask {
                    file_path: path.to_path_buf(),
                    url,
                });
            }
            _ => {
                let url = format!("/{}", relative.display());
                files.push(FileTask {
                    file_path: path.to_path_buf(),
                    url,
                });
            }
        }
    }

    Ok((pages, files))
}

/// Extract metadata from all page tasks in parallel
async fn extract_all_metadata(mut tasks: Vec<PageTask>) -> Result<Vec<PageTask>> {
    let mut set = JoinSet::new();

    for (index, task) in tasks.iter().enumerate() {
        let path = task.file_path.clone();
        set.spawn(async move {
            let meta = typst::query_page_meta(&path).await;
            let prefixes = typst::query_prefixes(&path).await;
            (index, meta, prefixes)
        });
    }

    while let Some(result) = set.join_next().await {
        let (index, meta_result, prefixes_result) = result?;

        match meta_result {
            Ok(Some(meta)) => tasks[index].meta = Some(meta),
            Ok(None) => {
                bail!(
                    "{}: missing #metadata((title: ..)) <page>",
                    tasks[index].file_path.display()
                );
            }
            Err(e) => {
                bail!("{}: {e}", tasks[index].file_path.display());
            }
        }

        match prefixes_result {
            Ok(prefixes) => tasks[index].query_prefixes = prefixes,
            Err(e) => {
                eprintln!(
                    "{}: WARN: failed to extract queries: {e}",
                    tasks[index].file_path.display()
                );
            }
        }
    }

    Ok(tasks)
}

fn build_index(tasks: &[PageTask]) -> Vec<PageMetaWithUrl> {
    tasks
        .iter()
        .filter_map(|task| {
            task.meta
                .as_ref()
                .map(|meta| PageMetaWithUrl::from_meta(task.url.clone(), meta))
        })
        .collect()
}

fn resolve_queries(
    tasks: &[PageTask],
    index: &[PageMetaWithUrl],
) -> FxHashMap<String, Vec<Vec<PageMetaWithUrl>>> {
    let mut resolved: FxHashMap<String, Vec<Vec<PageMetaWithUrl>>> =
        HashMap::with_hasher(FxBuildHasher);

    for task in tasks {
        if task.query_prefixes.is_empty() {
            continue;
        }

        let results: Vec<Vec<PageMetaWithUrl>> = task
            .query_prefixes
            .iter()
            .map(|prefix| {
                let norm_prefix = if prefix.starts_with('/') {
                    prefix.clone()
                } else {
                    format!("/{prefix}")
                };

                index
                    .iter()
                    .filter(|page| page.url.starts_with(&norm_prefix) && page.url != task.url)
                    .cloned()
                    .collect()
            })
            .collect();

        resolved.insert(task.url.clone(), results);
    }

    resolved
}

/// `content/foo/bar.typ` → `/foo/bar`
/// `content/index.typ` → `/`
/// `content/foo/index.typ` → `/foo`
pub fn file_path_to_url(content_dir: &Path, file_path: &Path) -> String {
    let relative = file_path
        .strip_prefix(content_dir)
        .expect("file_path should be under content_dir");

    let without_ext = relative.with_extension("");
    let path_str = without_ext.to_str().expect("path should be valid UTF-8");

    if path_str == "index" {
        "/".to_string()
    } else if let Some(stripped) = path_str.strip_suffix("/index") {
        format!("/{stripped}")
    } else if path_str.ends_with("/index") {
        format!("/{}", &path_str[..path_str.len() - 6])
    } else {
        format!("/{path_str}")
    }
}

pub fn is_hidden_path(path: &Path) -> bool {
    path.components()
        .any(|c| c.as_os_str().to_str().is_some_and(|s| s.starts_with('_')))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_path_to_url() {
        let content = Path::new("./content");

        assert_eq!(
            file_path_to_url(content, Path::new("./content/index.typ")),
            "/"
        );
        assert_eq!(
            file_path_to_url(content, Path::new("./content/blog.typ")),
            "/blog"
        );
        assert_eq!(
            file_path_to_url(content, Path::new("./content/blog/foo.typ")),
            "/blog/foo"
        );
        assert_eq!(
            file_path_to_url(content, Path::new("./content/blog/igloo/ecs.typ")),
            "/blog/igloo/ecs"
        );
        assert_eq!(
            file_path_to_url(content, Path::new("./content/projects/index.typ")),
            "/projects"
        );
    }

    #[test]
    fn test_is_hidden_path() {
        assert!(is_hidden_path(Path::new("content/_shared/template.typ")));
        assert!(is_hidden_path(Path::new("_foo/bar.typ")));
        assert!(!is_hidden_path(Path::new("content/blog/foo.typ")));
        assert!(!is_hidden_path(Path::new("content/index.typ")));
    }
}
