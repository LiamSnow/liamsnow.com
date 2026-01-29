use anyhow::{Context, Result};
use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri, header},
    response::IntoResponse,
    routing::get,
};
use clap::Parser;
use either::Either;
use mime_guess::{Mime, mime};
use rustc_hash::{FxBuildHasher, FxHashMap};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

mod sitemap;
mod typst;

const ROUTES_FILE: &str = "routes.toml";
const CONTENT_DIR: &str = "./content";

#[derive(Deserialize)]
struct Config {
    routes: Vec<ConfigRoute>,
}

#[derive(Deserialize)]
struct ConfigRoute {
    path: String,
    file: Option<String>,
    /// nest directory using another routes.toml file
    nest_dir: Option<String>,
    /// nest all files in directory
    auto_nest_dir: Option<String>,
}

struct Route {
    content: Either<String, Vec<u8>>,
    mime: Mime,
}

struct AppState {
    /// url path -> Route
    routes: FxHashMap<String, Route>,
    sitemap: String,
}

#[derive(Parser, Debug)]
#[command(name = "web-server")]
#[command(about = "A simple web server for blogs and projects", long_about = None)]
struct Args {
    // Hostname of IP address to bind to
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,

    // Port number (1-65535)
    #[arg(short, long, default_value_t = 3232)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let host = format!("{}:{}", args.address, args.port);

    println!("Generating pages...");

    let routes = generate("", &PathBuf::from(CONTENT_DIR))?;

    let sitemap = sitemap::generate(&routes);

    let state = Arc::new(AppState { routes, sitemap });

    let app = Router::new()
        .route("/sitemap.xml", get(serve_sitemap))
        .fallback(serve_page)
        .with_state(state)
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind(&host).await.unwrap();
    println!("Hosting at {host}!");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn generate(base_path: &str, dir: &PathBuf) -> Result<FxHashMap<String, Route>> {
    let config_path = dir.join(ROUTES_FILE);
    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    let config: Config = toml::from_str(&config_str)?;

    let mut routes = HashMap::with_capacity_and_hasher(config.routes.len(), FxBuildHasher);

    for route in config.routes {
        let path = if base_path.is_empty() {
            route.path.clone()
        } else {
            format!("{}{}", base_path, route.path)
        };

        // direct file
        if let Some(file) = route.file {
            let file_path = dir.join(&file);
            process_file(&mut routes, &path, &file_path)?;
        }

        // nest dir using another routes.toml file
        if let Some(subdir) = route.nest_dir {
            let subdir_path = dir.join(&subdir);
            let nested_routes = generate(&path, &subdir_path)?;
            routes.extend(nested_routes);
        }

        // nest all files in dir
        if let Some(subdir) = route.auto_nest_dir {
            use walkdir::WalkDir;

            let subdir_path = dir.join(&subdir);

            for entry in WalkDir::new(&subdir_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if !entry.file_type().is_file() {
                    continue;
                }

                let file_path = entry.path();
                let relative = file_path.strip_prefix(&subdir_path)?;
                let url_segment = relative
                    .to_str()
                    .context("Invalid UTF-8 in path")?
                    .replace('\\', "/");

                let full_path = format!("{}/{}", path.trim_end_matches('/'), url_segment);
                process_file(&mut routes, &full_path, &file_path.to_path_buf())?;
            }
        }
    }

    Ok(routes)
}

fn process_file(
    routes: &mut FxHashMap<String, Route>,
    url_path: &str,
    file_path: &PathBuf,
) -> Result<()> {
    match file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("typ") => {
            println!("Compiling `{}`", file_path.to_string_lossy());
            let content = typst::compile(file_path)?;
            routes.insert(
                url_path.trim_end_matches(".typ").to_string(),
                Route {
                    content: Either::Left(content),
                    mime: mime::TEXT_HTML_UTF_8,
                },
            );
        }
        ext @ Some("sass") | ext @ Some("scss") | ext @ Some("css") => {
            println!("Compiling `{}`", file_path.to_string_lossy());
            let opts = grass::Options::default().style(grass::OutputStyle::Compressed);
            let content = grass::from_path(file_path, &opts)?;
            let new_path = url_path.trim_end_matches(&format!(".{}", ext.unwrap()));
            routes.insert(
                format!("{new_path}.css"),
                Route {
                    content: Either::Left(content),
                    mime: mime::TEXT_CSS,
                },
            );
        }
        _ => {
            println!("Linking `{}`", file_path.to_string_lossy());
            let bytes = fs::read(file_path)?;
            let mime = mime_guess::from_path(file_path).first_or_text_plain();
            routes.insert(
                url_path.to_string(),
                Route {
                    content: Either::Right(bytes),
                    mime,
                },
            );
        }
    };

    Ok(())
}

async fn serve_page(uri: Uri, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.routes.get(uri.path()) {
        Some(route) => match &route.content {
            Either::Left(str) => (
                [(header::CONTENT_TYPE, route.mime.to_string())],
                str.clone(),
            )
                .into_response(),
            Either::Right(bytes) => (
                [(header::CONTENT_TYPE, route.mime.to_string())],
                bytes.clone(),
            )
                .into_response(),
        },

        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

async fn serve_sitemap(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/xml")],
        state.sitemap.clone(),
    )
}
