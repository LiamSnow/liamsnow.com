use std::{collections::HashMap, sync::OnceLock};

use axum::{
    Router,
    extract::Path,
    routing::{get, get_service},
};
use clap::Parser;
use post::PostCollection;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

mod home;
mod markdown;
mod post;
mod scss;
mod sitemap;
mod template;

static BLOGS: OnceLock<PostCollection> = OnceLock::new();
static PROJECTS: OnceLock<PostCollection> = OnceLock::new();
static SITEMAP: OnceLock<String> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(name = "web-server")]
#[command(about = "A simple web server for blogs and projects", long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3232)]
    port: u16,
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,
    /// working directory, no trailing slash
    /// For example "." or "/home/liams"
    #[arg(short, long, default_value = ".")]
    working_directory: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let host = format!("{}:{}", args.address, args.port);

    println!("Generating pages...");

    // generate pages from markdown files
    let (blogs_collection, recent_blogs) =
        PostCollection::new(&args.working_directory, "Blog".to_string());
    let (projects_collection, recent_projects) =
        PostCollection::new(&args.working_directory, "Projects".to_string());
    let sitemap = sitemap::generate(&blogs_collection, &projects_collection);

    // init shared structure here so we can unwrap later
    BLOGS.get_or_init(|| blogs_collection);
    PROJECTS.get_or_init(|| projects_collection);
    SITEMAP.get_or_init(|| sitemap);

    // generate homepage
    home::init(recent_projects, recent_blogs);

    // watch scss files if in dev mode
    #[cfg(feature = "dev")]
    {
        scss::watch();
    }

    let static_dir = format!("{}/static", args.working_directory);

    let app = Router::new()
        .route("/", get(home::get_home))
        .route("/blog", get(BLOGS.get().unwrap().index.clone()))
        .route(
            "/blog/{id}",
            get(|Path(params): Path<HashMap<String, String>>| async {
                BLOGS.get().unwrap().get_post(params)
            }),
        )
        .route("/projects", get(PROJECTS.get().unwrap().index.clone()))
        .route(
            "/projects/{id}",
            get(|Path(params): Path<HashMap<String, String>>| async {
                PROJECTS.get().unwrap().get_post(params)
            }),
        )
        .route(
            "/robots.txt",
            get_service(ServeFile::new(format!("{static_dir}/robots.txt"))),
        )
        .route(
            "/sitemap.xml",
            get(|| async {
                axum::response::Response::builder()
                    .header("Content-Type", "application/xml")
                    .body(SITEMAP.get().unwrap().clone())
                    .unwrap()
            }),
        )
        .route(
            "/blog/rss.xml",
            get(|| async {
                axum::response::Response::builder()
                    .header("Content-Type", "application/rss+xml")
                    .body(BLOGS.get().unwrap().rss.clone())
                    .unwrap()
            }),
        )
        .route(
            "/projects/rss.xml",
            get(|| async {
                axum::response::Response::builder()
                    .header("Content-Type", "application/rss+xml")
                    .body(PROJECTS.get().unwrap().rss.clone())
                    .unwrap()
            }),
        )
        .route(
            "/favicon.ico",
            get_service(ServeFile::new(format!("{static_dir}/favicon.ico"))),
        )
        .nest_service(
            "/static",
            axum::Router::new()
                .fallback_service(ServeDir::new(static_dir))
                .layer(SetResponseHeaderLayer::if_not_present(
                    axum::http::header::CACHE_CONTROL,
                    axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
                )),
        )
        .layer(CompressionLayer::new())
        .fallback("404");

    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();
    println!("Hosting at {host}!");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
