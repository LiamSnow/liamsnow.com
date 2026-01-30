use anyhow::Result;
use arc_swap::ArcSwap;
use axum::{
    Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, Uri, header},
    response::{IntoResponse, Response},
    routing::get,
};
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::AppState;
use crate::compiler::Route;

pub async fn run(state: Arc<ArcSwap<AppState>>, address: &str, port: u16) -> Result<()> {
    let host = format!("{address}:{port}");

    let app = Router::new()
        .route("/sitemap.xml", get(serve_sitemap))
        .fallback(serve_page)
        .with_state(state);

    let listener = TcpListener::bind(&host).await.unwrap();
    println!("Hosting at {host}!");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn accepts_brotli(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|s| s.contains("br"))
}

fn serve_route(route: &Route, use_br: bool) -> (HeaderValue, Option<HeaderValue>, Bytes) {
    if use_br {
        if let Some(br) = &route.content_br {
            return (
                route.content_type.clone(),
                Some(HeaderValue::from_static("br")),
                br.clone(),
            );
        }
    }
    (route.content_type.clone(), None, route.content_identity.clone())
}

fn build_response(content_type: HeaderValue, encoding: Option<HeaderValue>, body: Bytes) -> Response {
    if let Some(enc) = encoding {
        (
            [
                (header::CONTENT_TYPE, content_type),
                (header::CONTENT_ENCODING, enc),
            ],
            body,
        )
            .into_response()
    } else {
        ([(header::CONTENT_TYPE, content_type)], body).into_response()
    }
}

async fn serve_page(
    uri: Uri,
    headers: HeaderMap,
    State(state): State<Arc<ArcSwap<AppState>>>,
) -> Response {
    let state = state.load();
    match state.routes.get(uri.path()) {
        Some(route) => {
            let (ct, enc, body) = serve_route(route, accepts_brotli(&headers));
            build_response(ct, enc, body)
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn serve_sitemap(
    headers: HeaderMap,
    State(state): State<Arc<ArcSwap<AppState>>>,
) -> Response {
    let state = state.load();
    let (ct, enc, body) = serve_route(&state.sitemap, accepts_brotli(&headers));
    build_response(ct, enc, body)
}
