use crate::indexer::meta::{PAGE_KEY, QUERY_KEY};
use anyhow::{Context, Result, bail};
use mime_guess::{Mime, mime};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use typst::foundations::{Bytes, Dict, Value};
use typst::syntax::{FileId, Source, VirtualPath};

mod meta;

#[derive(Debug)]
pub struct FileSlot {
    pub url: String,
    pub hidden: bool,
    pub mime: Mime,
    pub file: Bytes,
    pub ty: SlotType,
}

#[derive(Debug)]
pub enum SlotType {
    Typst(TypstSlot),
    Scss,
    Other,
}

#[derive(Debug)]
pub struct TypstSlot {
    pub source: Source,
    /// None for hidden files
    pub page_meta: Option<Dict>,
    pub queries: Option<Dict>,
}

struct WalkEntry {
    path: PathBuf,
    rootless: PathBuf,
}

pub type Slots = FxHashMap<FileId, FileSlot>;
pub type MetaMap = BTreeMap<String, Dict>;

/// Indexes root directory
///  1. recursively walk the directory, finding all files
///  2. read each file + grab metadata from typst files
pub fn run(root: &Path) -> Result<(Slots, MetaMap)> {
    println!("  Walking...");
    let entries = walk(root)?;

    println!("  Reading...");
    let (slots, metamap) = read_and_parse(entries)?;

    Ok((slots, metamap))
}

fn walk(root: &Path) -> Result<Vec<WalkEntry>> {
    let mut entries = Vec::new();
    let mut stack = vec![fs::read_dir(root)?];

    while let Some(iter) = stack.last_mut() {
        let Some(entry) = iter.next() else {
            stack.pop();
            continue;
        };
        let entry = entry?;

        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            stack.push(fs::read_dir(entry.path())?);
            continue;
        }

        if file_type.is_file() {
            let path = entry.path();
            let rootless = path
                .strip_prefix(root)
                .context("path not in root")?
                .to_path_buf();
            entries.push(WalkEntry { path, rootless });
        }
    }

    Ok(entries)
}

fn read_and_parse(entries: Vec<WalkEntry>) -> Result<(Slots, MetaMap)> {
    let results = entries
        .into_par_iter()
        .map(|entry| {
            let vp = VirtualPath::new(&entry.rootless);
            let id = FileId::new(None, vp);
            let slot = FileSlot::new(id, entry)?;
            Ok((id, slot))
        })
        .collect::<Result<Vec<(FileId, FileSlot)>>>()?;

    let mut slots = HashMap::with_capacity_and_hasher(results.len(), FxBuildHasher);
    let mut metamap = BTreeMap::new();

    for (id, slot) in results {
        if let SlotType::Typst(typst) = &slot.ty
            && let Some(page_meta) = &typst.page_meta
        {
            metamap.insert(slot.url.clone(), page_meta.clone());
        }
        slots.insert(id, slot);
    }

    Ok((slots, metamap))
}

impl FileSlot {
    fn new(id: FileId, entry: WalkEntry) -> Result<Self> {
        let file = std::fs::read(&entry.path)?;

        let rootless_str = entry
            .rootless
            .to_str()
            .with_context(|| format!("`{:?}`'s path is not valid UTF-8", entry.path))?;
        let url = make_url(rootless_str);
        let hidden = is_hidden(rootless_str);

        let ext = entry
            .path
            .extension()
            .unwrap_or_default()
            .to_str()
            .with_context(|| format!("File `{:?}`'s extension is not valid UTF-8", entry.path))?;

        let mime = match ext {
            "typ" => mime::TEXT_HTML_UTF_8,
            "scss" => mime::TEXT_CSS_UTF_8,
            ext => mime_guess::from_ext(ext).first_or_text_plain(),
        };

        let ty = match ext {
            "typ" => SlotType::Typst(TypstSlot::new(id, &file, hidden, &url)?),
            "scss" => SlotType::Scss,
            _ => SlotType::Other,
        };

        Ok(FileSlot {
            url,
            hidden,
            mime,
            file: Bytes::new(file),
            ty,
        })
    }
}

impl TypstSlot {
    fn new(id: FileId, file: &[u8], hidden: bool, url: &str) -> Result<Self> {
        let text = String::from_utf8_lossy(file).to_string();
        let source = Source::new(id, text);

        if hidden {
            return Ok(TypstSlot {
                source,
                page_meta: None,
                queries: None,
            });
        }

        let text = source.as_ref();
        let mut all_meta = meta::parse(text)?;

        let Some(mut page_meta) = all_meta.remove(PAGE_KEY) else {
            bail!("`{id:?}` is missing `#metadata((..)) <page>`");
        };

        page_meta.insert("url".into(), Value::Str(url.into()));

        Ok(TypstSlot {
            source,
            page_meta: Some(page_meta),
            queries: all_meta.remove(QUERY_KEY),
        })
    }
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
        assert_eq!(make_url("index.typ"), "/");
        assert_eq!(make_url("cat/index.typ"), "/cat");
        assert_eq!(make_url("cat/dog.typ"), "/cat/dog");
        assert_eq!(make_url("style.scss"), "/style.css");
        assert_eq!(make_url("robots.txt"), "/robots.txt");
        assert_eq!(make_url("img/photo.png"), "/img/photo.png");
    }

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden("_drafts/post.typ"));
        assert!(is_hidden("blog/_hidden/post.typ"));
        assert!(!is_hidden("blog/post.typ"));
        assert!(!is_hidden("underscored_name/post.typ"));
    }
}
