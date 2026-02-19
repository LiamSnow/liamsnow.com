use crate::{CONFIG, indexer::PageMeta};
use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use tokio::process::Command;

/// Get value of `#metadata((title: "..", ..)) <page>` from a typst file
pub async fn get_metadata(source_path: &PathBuf) -> Result<PageMeta> {
    let cfg = CONFIG.get().unwrap();

    let out = Command::new(&cfg.typst)
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
        .arg(&cfg.content_dir)
        .arg("--jobs")
        .arg("1")
        .output()
        .await?;

    if out.stdout.is_empty() {
        bail!("{}", String::from_utf8_lossy(&out.stderr));
    }

    serde_json::from_slice(&out.stdout).context("failed to parse page metadata")
}

pub async fn compile(
    source_path: &PathBuf,
    url: &str,
    page_meta: &PageMeta,
    query_result: String,
) -> Result<String> {
    let cfg = CONFIG.get().unwrap();
    let page_json = serde_json::to_string(page_meta)?;

    let out = Command::new(&cfg.typst)
        .arg("compile")
        .arg(source_path)
        .arg("--features")
        .arg("html")
        .arg("--format")
        .arg("html")
        .arg("--root")
        .arg(&cfg.content_dir)
        .arg("--input")
        .arg(format!("path={url}"))
        .arg("--input")
        .arg(format!("page={page_json}"))
        .arg("--input")
        .arg(format!("query={query_result}"))
        .arg("--jobs")
        .arg("1")
        .arg("-")
        .output()
        .await?;

    if out.stdout.is_empty() {
        bail!("{}", String::from_utf8_lossy(&out.stderr));
    }

    let mut html = String::from_utf8(out.stdout).context("typst output was not valid UTF-8")?;

    if cfg.watch {
        html = html.replacen("</head>", &format!(r#"
            <script>
                (function() {{                                                                              
                    const ws = new WebSocket(`ws://{}:{}`);                              
                    ws.onmessage = () => location.reload();                                                  
                    ws.onclose = () => setTimeout(() => location.reload(), 1000);                            
                }})();
            </script>
            </head>
            "#, cfg.watch_address, cfg.watch_port), 1);
    }

    Ok(html)
}
