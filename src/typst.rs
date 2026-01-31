use crate::CONTENT_DIR;
use crate::indexer::{PageMeta, PageMetaWithUrl};
use crate::watcher::WATCH_ADDR;
use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::process::Command;

static BINARY_PATH: OnceLock<String> = OnceLock::new();

pub fn set_binary_path(path: &str) {
    BINARY_PATH.set(path.to_string()).ok();
}

fn get_binary() -> &'static str {
    BINARY_PATH.get().map(|s| s.as_str()).unwrap_or("typst")
}

/// Get value of `#metadata((title: "..", ..)) <page>` from a typst file
pub async fn query_page_meta(source_path: &PathBuf) -> Result<Option<PageMeta>> {
    let out = Command::new(get_binary())
        .arg("query")
        .arg(source_path)
        .arg("<page>")
        .arg("--field")
        .arg("value")
        .arg("--one")
        .arg("--format")
        .arg("json")
        .arg("--features")
        .arg("html")
        .arg("--root")
        .arg(CONTENT_DIR)
        .output()
        .await?;

    if out.stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("label does not exist") || stderr.contains("no matches") {
            return Ok(None);
        }
        if !stderr.is_empty() {
            bail!("typst query error: {stderr}");
        }
        return Ok(None);
    }

    let meta: PageMeta =
        serde_json::from_slice(&out.stdout).context("failed to parse page metadata")?;
    Ok(Some(meta))
}

/// Get value of either:
///  - `#metadata("..") <query>`
///  - `#metadata(("..", "..", ..)) <query>`
pub async fn query_prefixes(source_path: &PathBuf) -> Result<Vec<String>> {
    let out = Command::new(get_binary())
        .arg("query")
        .arg(source_path)
        .arg("<query>")
        .arg("--field")
        .arg("value")
        .arg("--one")
        .arg("--format")
        .arg("json")
        .arg("--features")
        .arg("html")
        .arg("--root")
        .arg(CONTENT_DIR)
        .output()
        .await?;

    if out.stdout.is_empty() {
        return Ok(vec![]);
    }

    let value: Value =
        serde_json::from_slice(&out.stdout).context("failed to parse query prefixes")?;

    let prefixes = match value {
        Value::String(s) => vec![s],
        Value::Array(arr) => arr
            .into_iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => vec![],
    };

    Ok(prefixes)
}

pub async fn compile(
    source_path: &PathBuf,
    url: &str,
    page_meta: &PageMeta,
    query_results: &[Vec<PageMetaWithUrl>],
) -> Result<String> {
    let page_json = serde_json::to_string(page_meta)?;
    let query_json = serde_json::to_string(query_results)?;

    let out = Command::new(get_binary())
        .arg("compile")
        .arg(source_path)
        .arg("--features")
        .arg("html")
        .arg("--format")
        .arg("html")
        .arg("--root")
        .arg(CONTENT_DIR)
        .arg("--input")
        .arg(format!("path={url}"))
        .arg("--input")
        .arg(format!("page={page_json}"))
        .arg("--input")
        .arg(format!("query={query_json}"))
        .arg("-")
        .output()
        .await?;

    if out.stdout.is_empty() {
        bail!("{}", String::from_utf8_lossy(&out.stderr));
    }

    let mut html = String::from_utf8(out.stdout).context("typst output was not valid UTF-8")?;

    if let Some(addr) = WATCH_ADDR.get().unwrap() {
        html = html.replacen("</head>", &format!(r#"
            <script>
                (function() {{                                                                              
                    const ws = new WebSocket(`ws://{addr}`);                              
                    ws.onmessage = () => location.reload();                                                  
                    ws.onclose = () => setTimeout(() => location.reload(), 1000);                            
                }})();
            </script>
            </head>
            "#), 1);
    }

    Ok(html)
}
