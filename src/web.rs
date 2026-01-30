use anyhow::Result;
use arc_swap::ArcSwap;
use bytes::Bytes;
use http::{HeaderValue, Request, Response, StatusCode, header};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::AppState;
use crate::compiler::Route;

pub async fn run(state: Arc<ArcSwap<AppState>>, address: &str, port: u16) -> Result<()> {
    let host = format!("{address}:{port}");
    let listener = TcpListener::bind(&host).await?;
    println!("Hosting at {host}!");

    loop {
        let (stream, _) = listener.accept().await?;
        let state = state.clone();

        tokio::spawn(async move {
            let _ = http1::Builder::new()
                .serve_connection(
                    TokioIo::new(stream),
                    service_fn(move |req| handle(req, state.clone())),
                )
                .await;
        });
    }
}

async fn handle(
    req: Request<Incoming>,
    state: Arc<ArcSwap<AppState>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let state = state.load();
    let use_br = accepts_brotli(req.headers());

    let route = match req.uri().path() {
        "/sitemap.xml" => Some(&state.sitemap),
        path => state.routes.get(path),
    };

    let response = match route {
        Some(route) => build_response(route, use_br),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::new()))
            .unwrap(),
    };

    Ok(response)
}

fn accepts_brotli(headers: &http::HeaderMap) -> bool {
    headers
        .get(header::ACCEPT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|s| s.contains("br"))
}

fn build_response(route: &Route, use_br: bool) -> Response<Full<Bytes>> {
    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, route.content_type.clone());

    let body = if use_br {
        if let Some(br) = &route.content_br {
            builder = builder.header(header::CONTENT_ENCODING, HeaderValue::from_static("br"));
            br.clone()
        } else {
            route.content_identity.clone()
        }
    } else {
        route.content_identity.clone()
    };

    builder.body(Full::new(body)).unwrap()
}
