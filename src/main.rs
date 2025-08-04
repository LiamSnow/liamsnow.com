use std::{collections::HashMap, sync::OnceLock};

use axum::{
    Router,
    extract::Path,
    routing::{get, get_service},
};
use post::PostCollection;
use tower_http::services::{ServeDir, ServeFile};

mod home;
mod post;
mod template;

static BLOGS: OnceLock<PostCollection> = OnceLock::new();
static PROJECTS: OnceLock<PostCollection> = OnceLock::new();

#[tokio::main]
async fn main() {
    let (blogs_collection, recent_blogs) = PostCollection::new("Blog".to_string());
    let (projects_collection, recent_projects) = PostCollection::new("Projects".to_string());

    BLOGS.get_or_init(|| blogs_collection);
    PROJECTS.get_or_init(|| projects_collection);

    home::init(recent_projects, recent_blogs);

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
        .nest_service("/static", ServeDir::new("static"))
        .fallback("404");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3232").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
