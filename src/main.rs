use anyhow::{Context, Result};
use axum::{
    Router,
    extract::State,
    http::{HeaderName, HeaderValue, StatusCode, Uri, header},
    response::Html,
    routing::{get, get_service},
};
use clap::Parser;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use typst_as_lib::TypstEngine;

mod sitemap;

const CONTENT_DIR: &str = "./content";
const STATIC_DIR: &str = "./static";

#[derive(Deserialize)]
struct Config {
    routes: Vec<Route>,
}

#[derive(Deserialize)]
struct Route {
    path: String,
    file: Option<String>,
    dir: Option<String>,
}

struct ResolvedRoute {
    url_path: String,
    file_path: String,
}

struct AppState {
    pages: HashMap<String, String>,
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

    let routes = resolve_routes("", &PathBuf::from(CONTENT_DIR))?;

    let pages = compile_routes(&routes).await?;
    println!("Compiled {} pages:", pages.len());
    for path in pages.keys() {
        println!("  {}", path);
    }

    let url_paths: Vec<String> = pages.keys().cloned().collect();
    let sitemap = sitemap::generate(&url_paths);

    let state = Arc::new(AppState { pages, sitemap });

    let app = Router::new()
        .route("/sitemap.xml", get(serve_sitemap))
        .nest_service(
            "/static",
            Router::new()
                .fallback_service(ServeDir::new(STATIC_DIR))
                .layer(SetResponseHeaderLayer::if_not_present(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=31536000, immutable"),
                )),
        )
        .route(
            "/robots.txt",
            get_service(ServeFile::new(format!("{STATIC_DIR}/robots.txt"))),
        )
        .route(
            "/favicon.ico",
            get_service(ServeFile::new(format!("{STATIC_DIR}/favicon.ico"))),
        )
        .fallback(serve_page)
        .with_state(state)
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind(&host).await.unwrap();
    println!("Hosting at {host}!");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn resolve_routes(base_path: &str, dir: &PathBuf) -> Result<Vec<ResolvedRoute>> {
    let config_path = dir.join("routes.toml");
    let config_str = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    let config: Config = toml::from_str(&config_str)?;

    let mut resolved = Vec::new();

    for route in config.routes {
        let full_path = if base_path.is_empty() {
            route.path.clone()
        } else {
            format!("{}{}", base_path, route.path)
        };

        if let Some(file) = route.file {
            let file_path = dir.join(&file);
            resolved.push(ResolvedRoute {
                url_path: full_path,
                file_path: file_path.to_string_lossy().to_string(),
            });
        } else if let Some(subdir) = route.dir {
            let subdir_path = dir.join(&subdir);

            let nested_base = full_path
                .split(':')
                .next()
                .unwrap_or(&full_path)
                .trim_end_matches('/');

            let nested_routes = resolve_routes(nested_base, &subdir_path)?;
            resolved.extend(nested_routes);
        }
    }

    Ok(resolved)
}

async fn compile_routes(routes: &[ResolvedRoute]) -> Result<HashMap<String, String>> {
    let mut pages = HashMap::new();

    for route in routes {
        let html = compile_typst(&route.file_path)?;
        pages.insert(route.url_path.clone(), html);
    }

    Ok(pages)
}

fn compile_typst(file: &str) -> Result<String> {
    let file_path = std::path::Path::new(file);
    let parent_dir = file_path
        .parent()
        .unwrap_or(std::path::Path::new(CONTENT_DIR));

    let file_contents = fs::read_to_string(file)?;

    let template = TypstEngine::builder()
        .main_file(file_contents)
        .with_file_system_resolver(parent_dir)
        .with_package_file_resolver()
        .build();

    let doc = template.compile().output.unwrap();

    let html = typst_html::html(&doc).unwrap();

    Ok(html)
}

async fn serve_page(
    uri: Uri,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    state
        .pages
        .get(uri.path())
        .map(|html| Html(html.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

async fn serve_sitemap(
    State(state): State<Arc<AppState>>,
) -> ([(HeaderName, &'static str); 1], String) {
    (
        [(header::CONTENT_TYPE, "application/xml")],
        state.sitemap.clone(),
    )
}
