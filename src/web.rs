use anyhow::Result;
use bytes::Bytes;
use http::{HeaderValue, Method, Request, Response, StatusCode, header};
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use tokio::net::TcpListener;

use crate::compiler::Route;
use crate::{CONFIG, ROUTING_TABLE, update};

const UPDATE_PATH: &str = "_update";

pub async fn run() -> Result<()> {
    let config = CONFIG.get().unwrap();
    let host = format!("{}:{}", config.address, config.port);
    let listener = TcpListener::bind(&host).await?;
    println!("Hosting at {host}!");

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = http1::Builder::new()
                .serve_connection(TokioIo::new(stream), service_fn(move |req| handle(req)))
                .await;
        });
    }
}

async fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    if req.method() == Method::POST && req.uri().path() == UPDATE_PATH {
        return Ok(update::handle(req).await);
    }

    let routing_table = ROUTING_TABLE.load();
    let route = routing_table.get(req.uri().path());

    let response = match route {
        Some(route) => build_response(route, accepts_brotli(req.headers())),
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

    if let Some(cc) = &route.cache_control {
        builder = builder.header(header::CACHE_CONTROL, cc.clone());
    }

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
