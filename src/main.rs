use axum::{
    routing::{get, get_service},
    Router,
};
use maud::{html, Markup};
use tower_http::services::{ServeDir, ServeFile};

mod page;
mod home;
mod blog;
mod markdown;

async fn handle_404() -> Markup {
    html! {
        h1 { "404" }
    }
}

#[tokio::main]
async fn main() {
    home::init();
    blog::init();

    let app = Router::new()
        .route("/", get(home::get_home))
        .route("/blog", get(blog::get_home))
        .route("/blog/:id", get(blog::get_blog))
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./static/robots.txt")),
        )
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("./static/favicon.ico")),
        )
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handle_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
