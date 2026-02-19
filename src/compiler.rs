use crate::indexer::{Task, TaskType};
use crate::{RoutingTable, sitemap, typst};
use anyhow::{Context, Result, bail};
use brotli::{BrotliCompress, enc::BrotliEncoderParams};
use bytes::Bytes;
use http::HeaderValue;
use mime_guess::mime;
use rustc_hash::FxBuildHasher;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::task::JoinSet;

pub struct Route {
    pub content_br: Option<Bytes>,
    pub content_identity: Bytes,
    pub content_type: HeaderValue,
    pub cache_control: Option<HeaderValue>,
}

pub async fn compile(index: BTreeMap<String, Task>) -> Result<RoutingTable> {
    let mut set = JoinSet::new();
    let mut routing_table = HashMap::with_capacity_and_hasher(index.len(), FxBuildHasher);

    for (url, task) in index {
        match task.ty {
            TaskType::Page { meta, query_result } => {
                set.spawn(async move {
                    let content = typst::compile(&task.path, &url, &meta, query_result).await?;
                    Ok((
                        url,
                        Route::from_string(content, mime::TEXT_HTML_UTF_8, None),
                    ))
                });
            }
            TaskType::File => {
                set.spawn(process_file(url, task.path));
            }
        }
    }

    while let Some(result) = set.join_next().await {
        let (url, route) = result??;
        routing_table.insert(url, route);
    }

    sitemap::generate(&mut routing_table);

    Ok(routing_table)
}

async fn process_file(url: String, path: PathBuf) -> Result<(String, Route)> {
    // SAFETY: extension existance checked by indexer
    let ext = path.extension().unwrap().to_string_lossy();

    let result = match ext.as_ref() {
        "scss" | "sass" | "css" => process_css(&path).await,
        "js" | "txt" | "md" | "csv" => process_static(&path, true).await,
        _ => process_static(&path, false).await,
    };

    match result {
        Ok(route) => Ok((url, route)),
        Err(e) => {
            bail!("{}: {e}", path.display());
        }
    }
}

async fn process_css(path: &Path) -> Result<Route> {
    let opts = grass::Options::default().style(grass::OutputStyle::Compressed);
    let content = grass::from_path(path, &opts)?;
    Ok(Route::from_string(content, mime::TEXT_CSS, None))
}

async fn process_static(path: &Path, compress: bool) -> Result<Route> {
    let bytes = fs::read(path).await.context("failed to read file")?;
    let mime = mime_guess::from_path(path).first_or_text_plain();

    let cache_control = (mime.type_() == "font")
        .then(|| HeaderValue::from_static("public, max-age=31536000, immutable"));

    Ok(if compress {
        Route::from_bytes(bytes, mime, cache_control)
    } else {
        Route::from_bytes_precompressed(bytes, mime, cache_control)
    })
}

impl Route {
    pub fn from_bytes(
        content: Vec<u8>,
        mime: impl ToString,
        cache_control: Option<HeaderValue>,
    ) -> Self {
        Self {
            content_br: Some(compress_brotli(&content)),
            content_identity: Bytes::from(content),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
            cache_control,
        }
    }

    pub fn from_bytes_precompressed(
        content: Vec<u8>,
        mime: impl ToString,
        cache_control: Option<HeaderValue>,
    ) -> Self {
        Self {
            content_br: None,
            content_identity: Bytes::from(content),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
            cache_control,
        }
    }

    pub fn from_string(
        content: String,
        mime: impl ToString,
        cache_control: Option<HeaderValue>,
    ) -> Self {
        let bytes = content.into_bytes();
        Self {
            content_br: Some(compress_brotli(&bytes)),
            content_identity: Bytes::from(bytes),
            content_type: HeaderValue::from_str(&mime.to_string()).unwrap(),
            cache_control,
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
