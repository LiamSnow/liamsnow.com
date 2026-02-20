//! Walk the root file tree, read all files into memory,
//! extra typst metadata, and evaluate queries.
//!
//! We have to split this into three distinct steps:
//!  1. recursively walk file tree, reading each file
//!  2. extract metadata from typst files
//!      - requires compiling them, which requires #1
//!  3. evaluate queries
//!      - requires map of url -> metadata, from #2

use crate::CONFIG;
use crate::typst::LiamsWorld;
use anyhow::{Context, Result, bail};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::{BTreeMap, HashMap};
use std::ops::Bound;
use std::path::Path;
use std::sync::Arc;
use tokio::task::JoinSet;
use tokio::{fs, io};
use typst::foundations::{Array, Bytes, Dict, Value};
use typst::syntax::{FileId, Source, VirtualPath};

#[derive(Debug, Clone)]
pub struct FileSlot {
    pub id: FileId,
    pub url: String,
    pub ext: String,
    pub is_hidden: bool,
    pub source: Option<Source>,
    pub inputs: Option<Dict>,
    pub file: Bytes,
}

pub type Slots = FxHashMap<FileId, FileSlot>;
type MetaMap = BTreeMap<String, Dict>;

pub async fn run() -> Result<Slots> {
    let root = &CONFIG.get().unwrap().root;

    println!("  Walking and reading...");
    let (slots, typst_queue) = walk_and_read(root).await?;

    println!("  Extracing metadata...");
    let slots = Arc::new(slots);
    let (metamap, typst_queue) = extra_metadata(&slots, typst_queue).await?;
    let mut slots = Arc::into_inner(slots).unwrap();

    println!("  Evaluating queries...");
    eval_queries(&mut slots, metamap, typst_queue).await?;

    Ok(slots)
}

async fn walk_and_read(root: &Path) -> Result<(Slots, Vec<(FileId, String)>)> {
    let mut tasks: JoinSet<io::Result<(FileId, FileSlot)>> = JoinSet::new();
    let mut stack = vec![fs::read_dir(root).await?];

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

        let path = entry.path();
        if file_type.is_file()
            && let Some(ext) = path.extension()
        {
            let Some(ext) = ext.to_str() else {
                bail!("File extension `{ext:?}` is not valid UTF-8");
            };
            let ext = ext.to_string();

            let rootless = path
                .strip_prefix(root)
                .context("path not in root")?
                .to_path_buf();
            let rootless_str = rootless
                .to_str()
                .with_context(|| format!("`{path:?}` is not valid UTF-8"))?;
            let url = make_url(rootless_str);
            let is_hidden = is_hidden(rootless_str);

            tasks.spawn(async move {
                let bytes = tokio::fs::read(path).await?;
                let vp = VirtualPath::new(rootless);
                let id = FileId::new(None, vp);

                let source = (ext == "typ").then(|| {
                    // Remove UTF-8 BOM.
                    // let text = from_utf8(bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(&bytes))?;
                    let text = String::from_utf8_lossy(&bytes).to_string();
                    Source::new(id, text)
                });

                let slot = FileSlot {
                    id,
                    url,
                    ext,
                    is_hidden,
                    source,
                    inputs: None,
                    file: Bytes::new(bytes),
                };
                Ok((id, slot))
            });
        }
    }

    let mut slots = HashMap::with_capacity_and_hasher(tasks.len(), FxBuildHasher);
    let mut typst_queue = Vec::with_capacity(8);
    while let Some(result) = tasks.join_next().await {
        let (id, slot) = result??;
        if slot.source.is_some() && !slot.is_hidden {
            typst_queue.push((id, slot.url.clone()));
        }
        slots.insert(id, slot);
    }

    Ok((slots, typst_queue))
}

async fn extra_metadata(
    slots: &Arc<Slots>,
    typst_queue: Vec<(FileId, String)>,
) -> Result<(MetaMap, Vec<(FileId, Dict)>)> {
    let mut tasks = JoinSet::new();

    for (id, url) in typst_queue {
        let slots = slots.clone();
        tasks.spawn(async move {
            let mut world = LiamsWorld::new(id, slots, None);
            let doc = world.compile()?;

            let mut meta = match world.query(&doc, "<page>") {
                Ok(Value::Dict(m)) => m,
                Ok(v) => {
                    bail!("{id:?}: #metadata(..) <page> was `{v:?}` instead of dictionary");
                }
                Err(msg) => {
                    println!("{id:?}: error {}", msg.message());
                    for hint in msg.hints() {
                        println!("hint: {hint}");
                    }
                    bail!("bad query");
                }
            };

            meta.insert("url".into(), Value::Str(url.clone().into()));

            Ok((id, url, meta))
        });
    }

    let mut metamap = BTreeMap::new();
    let mut typst_queue = Vec::with_capacity(tasks.len());

    while let Some(result) = tasks.join_next().await {
        let (id, url, meta) = result??;

        metamap.insert(url, meta.clone());
        typst_queue.push((id, meta));
    }

    Ok((metamap, typst_queue))
}

async fn eval_queries(
    slots: &mut Slots,
    metamap: MetaMap,
    typst_queue: Vec<(FileId, Dict)>,
) -> Result<()> {
    for (id, mut meta) in typst_queue {
        let mut inputs = Dict::new();
        inputs.insert("page".into(), Value::Dict(meta.clone()));

        // find queries that need eval
        if let Ok(value) = meta.take("queries") {
            let Value::Array(array) = value else {
                bail!("{id:?}: #metadata(..) <page> `queries` key was not an array");
            };

            // ahhh typst i want to modify in place!!
            let mut result = Array::with_capacity(array.len());
            for value in array {
                let Value::Str(query) = value else {
                    bail!("{id:?} #metadata(..) <page> `queries` had non string element in array");
                };

                let mut end = query.clone().to_string();
                if let Some(last) = end.as_bytes().last().copied() {
                    end.pop();
                    end.push((last + 1) as char);
                }

                result.push(Value::Array(
                    metamap
                        .range::<str, _>((
                            Bound::Included(query.as_str()),
                            Bound::Excluded(end.as_str()),
                        ))
                        .map(|(_, meta)| Value::Dict(meta.clone()))
                        .collect(),
                ));
            }

            inputs.insert("query".into(), Value::Array(result));
        }

        // attach inputs to sources
        slots.get_mut(&id).unwrap().inputs = Some(inputs);
    }
    Ok(())
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
    } else if let Some(stem) = rel.strip_suffix(".scss") {
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
