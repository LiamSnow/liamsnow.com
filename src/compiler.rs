use anyhow::{Context, Result};
use brotli::{BrotliCompress, enc::BrotliEncoderParams};
use bytes::Bytes;
use http::HeaderValue;
use mime_guess::mime;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{collections::HashMap, fs};
use tokio::task::JoinSet;

use crate::indexer::{FileTask, Index};
use crate::typst;

pub struct Route {
    pub content_br: Option<Bytes>,
    pub content_identity: Bytes,
    pub content_type: HeaderValue,
}

impl Route {
    pub fn from_bytes(content: Vec<u8>, mime: impl ToString) -> Self {
        Self {
            content_br: Some(compress_brotli(&content)),
            content_identity: Bytes::from(content),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
        }
    }

    pub fn from_bytes_precompressed(content: Vec<u8>, mime: impl ToString) -> Self {
        Self {
            content_br: None,
            content_identity: Bytes::from(content),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
        }
    }

    pub fn from_string(content: String, mime: impl ToString) -> Self {
        let bytes = content.into_bytes();
        Self {
            content_br: Some(compress_brotli(&bytes)),
            content_identity: Bytes::from(bytes),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
        }
    }
}

fn compress_brotli(input: &[u8]) -> Bytes {
    let mut output = Vec::new();
    let params = BrotliEncoderParams {
        quality: 11,
        ..Default::default()
    };
    BrotliCompress(&mut &input[..], &mut output, &params).unwrap();
    Bytes::from(output)
}

pub async fn compile(index: Index) -> FxHashMap<String, Route> {
    println!("  Compiling...");

    let mut set = JoinSet::new();
    let mut routes =
        HashMap::with_capacity_and_hasher(index.pages.len() + index.files.len(), FxBuildHasher);

    // compile pages
    for task in index.pages {
        let query_results = index
            .resolved_queries
            .get(&task.url)
            .cloned()
            .unwrap_or_default();

        set.spawn(async move {
            let meta = task.meta.as_ref().expect("metadata should be populated");
            let result = typst::compile(&task.file_path, &task.url, meta, &query_results).await;

            match result {
                Ok(content) => Some((task.url, Route::from_string(content, mime::TEXT_HTML_UTF_8))),
                Err(e) => {
                    eprintln!("{}: {e}", task.file_path.display());
                    None
                }
            }
        });
    }

    // compile files
    for task in index.files {
        set.spawn(async move { process_file(task) });
    }

    while let Some(result) = set.join_next().await {
        if let Ok(Some((url, route))) = result {
            routes.insert(url, route);
        }
    }

    routes
}

fn process_file(task: FileTask) -> Option<(String, Route)> {
    let ext = task
        .file_path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);

    let result = match ext.as_deref() {
        Some("scss" | "sass" | "css") => process_css(&task),
        Some("js" | "txt" | "md" | "csv") => process_static(&task, true),
        _ => process_static(&task, false),
    };

    match result {
        Ok((url, route)) => Some((url, route)),
        Err(e) => {
            eprintln!("{}: {e}", task.file_path.display());
            None
        }
    }
}

fn process_css(task: &FileTask) -> Result<(String, Route)> {
    let opts = grass::Options::default().style(grass::OutputStyle::Compressed);
    let content =
        grass::from_path(&task.file_path, &opts).context("failed to compile css/scss/sass")?;
    Ok((
        task.url.clone(),
        Route::from_string(content, mime::TEXT_CSS),
    ))
}

fn process_static(task: &FileTask, compress: bool) -> Result<(String, Route)> {
    let bytes = fs::read(&task.file_path).context("failed to read file")?;
    let mime = mime_guess::from_path(&task.file_path).first_or_text_plain();
    let route = if compress {
        Route::from_bytes(bytes, mime)
    } else {
        Route::from_bytes_precompressed(bytes, mime)
    };
    Ok((task.url.clone(), route))
}
