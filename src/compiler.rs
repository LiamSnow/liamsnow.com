use anyhow::{Context, Result};
use axum::{body::Bytes, http::HeaderValue};
use brotli::{BrotliCompress, enc::BrotliEncoderParams};
use mime_guess::mime;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{collections::HashMap, fs};
use tokio::task::JoinSet;

use crate::routes::FileTask;
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

pub async fn compile(tasks: Vec<FileTask>) -> FxHashMap<String, Route> {
    let mut set = JoinSet::new();
    let mut routes = HashMap::with_capacity_and_hasher(tasks.len(), FxBuildHasher);
    for task in tasks {
        set.spawn(process_file(task));
    }

    while let Some(result) = set.join_next().await {
        if let Ok(Some((url_path, route))) = result {
            routes.insert(url_path, route);
        }
    }
    routes
}

async fn process_file(task: FileTask) -> Option<(String, Route)> {
    let ext = task
        .file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase);

    let result = match ext.as_deref() {
        Some("typ") => process_typst(&task).await,
        Some("sass") | Some("scss") | Some("css") => process_css(&task),
        Some("js" | "txt" | "md" | "csv") => process_static(&task, true),
        _ => process_static(&task, false),
    };

    match result {
        Ok((url_path, route)) => Some((url_path, route)),
        Err(e) => {
            eprintln!("{}: {e}", task.file_path.display());
            None
        }
    }
}

async fn process_typst(task: &FileTask) -> Result<(String, Route)> {
    let content = typst::compile(&task.file_path).await?;
    let url_path = task.url_path.trim_end_matches(".typ").to_string();
    Ok((url_path, Route::from_string(content, mime::TEXT_HTML_UTF_8)))
}

fn process_css(task: &FileTask) -> Result<(String, Route)> {
    let opts = grass::Options::default().style(grass::OutputStyle::Compressed);
    let content =
        grass::from_path(&task.file_path, &opts).context("failed to compile scss/sass")?;
    let url_path = task.url_path.rsplit_once('.').map_or_else(
        || format!("{}.css", task.url_path),
        |(base, _)| format!("{base}.css"),
    );
    Ok((url_path, Route::from_string(content, mime::TEXT_CSS)))
}

fn process_static(task: &FileTask, compress: bool) -> Result<(String, Route)> {
    let bytes = fs::read(&task.file_path).context("failed to read file")?;
    let mime = mime_guess::from_path(&task.file_path).first_or_text_plain();
    let route = if compress {
        Route::from_bytes(bytes, mime)
    } else {
        Route::from_bytes_precompressed(bytes, mime)
    };
    Ok((task.url_path.clone(), route))
}
