use anyhow::Result;
use arc_swap::ArcSwap;
use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri, header},
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

use crate::AppState;

pub async fn run(state: Arc<ArcSwap<AppState>>, address: &str, port: u16) -> Result<()> {
    let host = format!("{address}:{port}");

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

async fn serve_page(uri: Uri, State(state): State<Arc<ArcSwap<AppState>>>) -> impl IntoResponse {
    let state = state.load();
    match state.routes.get(uri.path()) {
        Some(route) => (route.headers.clone(), route.content.clone()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn serve_sitemap(State(state): State<Arc<ArcSwap<AppState>>>) -> impl IntoResponse {
    let state = state.load();
    (
        [(header::CONTENT_TYPE, "application/xml")],
        state.sitemap.clone(),
    )
}
