use std::{collections::HashMap, sync::OnceLock};

use axum::{
    Router,
    extract::Path,
    routing::{get, get_service},
};
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
mod template;

const HOST: &str = "0.0.0.0:3232";
static BLOGS: OnceLock<PostCollection> = OnceLock::new();
static PROJECTS: OnceLock<PostCollection> = OnceLock::new();

#[tokio::main]
async fn main() {
    println!("Generating pages...");

    // generate pages from markdown files
    let (blogs_collection, recent_blogs) = PostCollection::new("Blog".to_string());
    let (projects_collection, recent_projects) = PostCollection::new("Projects".to_string());

    // init shared structure here so we can unwrap later
    BLOGS.get_or_init(|| blogs_collection);
    PROJECTS.get_or_init(|| projects_collection);

    // generate homepage
    home::init(recent_projects, recent_blogs);

    // watch scss files if in dev mode
    #[cfg(feature = "dev")]
    {
        scss::watch();
    }

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
            get_service(ServeFile::new("./static/robots.txt")),
        )
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("./static/favicon.ico")),
        )
        .nest_service(
            "/static",
            axum::Router::new()
                .fallback_service(ServeDir::new("static"))
                .layer(SetResponseHeaderLayer::if_not_present(
                    axum::http::header::CACHE_CONTROL,
                    axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
                )),
        )
        .layer(CompressionLayer::new())
        .fallback("404");

    let listener = tokio::net::TcpListener::bind(HOST).await.unwrap();
    println!("Hosting at {HOST}!");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
