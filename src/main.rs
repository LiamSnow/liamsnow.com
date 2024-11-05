use std::{collections::HashMap, sync::OnceLock};

use axum::{
    extract::Path,
    routing::{get, get_service},
    Router,
};
use post::PostCollection;
use tower_http::services::{ServeDir, ServeFile};

mod template;
mod post;
mod home;

static BLOGS: OnceLock<PostCollection> = OnceLock::new();
static PROJECTS: OnceLock<PostCollection> = OnceLock::new();

#[tokio::main]
async fn main() {
    BLOGS.get_or_init(|| PostCollection::new("Blog".to_string()));
    PROJECTS.get_or_init(|| PostCollection::new("Projects".to_string()));
    home::init();

    let app = Router::new()
        .route("/", get(home::get_home))
        .route("/blog", get(BLOGS.get().unwrap().index.clone()))
        .route(
            "/blog/:id",
            get(|Path(params): Path<HashMap<String, String>>| async {
                BLOGS.get().unwrap().get_post(params)
            }),
        )
        .route("/projects", get(PROJECTS.get().unwrap().index.clone()))
        .route(
            "/projects/:id",
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
        .nest_service("/static", ServeDir::new("static"))
        .fallback("404");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
