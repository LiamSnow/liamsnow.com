use anyhow::Result;
use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::{BrotliCompress, enc::BrotliEncoderParams};
use mime_guess::Mime;
use mime_guess::mime::{APPLICATION, FONT, IMAGE, SVG, TEXT};
use std::io::Write;
use typst::syntax::FileId;
use xxhash_rust::xxh3::xxh3_64;

/// A pre-serialized response
/// Making zero-copy dispatching since 2026 😀
pub struct Route {
    /// brotli compress HTTP response
    pub brotli: Box<[u8]>,
    /// uncompressed HTTP response
    pub identity: Box<[u8]>,
    pub not_modified: Box<[u8]>,
    pub etag: Box<[u8]>,
}

macro_rules! empty_response {
    ($status:expr) => {
        concat!(
            "HTTP/1.1 ",
            $status,
            "\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        )
        .as_bytes()
    };
}

pub static BAD_REQUEST: &[u8] = empty_response!("400 Bad Request");
pub static NOT_FOUND: &[u8] = empty_response!("404 Not Found");
pub static METHOD_NOT_ALLOWED: &[u8] = empty_response!("405 Method Not Allowed");
pub static PAYLOAD_TOO_LARGE: &[u8] = empty_response!("413 Payload Too Large");
pub static OK: &[u8] = empty_response!("200 OK");
pub static UNAUTHORIZED: &[u8] = empty_response!("401 Unauthorized");

impl Route {
    pub fn compile(id: &FileId, content: Vec<u8>, mime: &Mime, fast: bool) -> Result<Self> {
        let cache_control = cache_control(mime);
        let brotli_settings = brotli_settings(mime, fast);

        let hash = xxh3_64(&content);
        let etag = format!("\"{hash:016x}\"");

        let identity = serialize(
            &content,
            mime.as_ref(),
            cache_control,
            brotli_settings.is_some(),
            None,
            &etag,
        )?;

        let brotli = match brotli_settings {
            Some(settings) => {
                let compressed = compress_brotli(&content, settings);
                if compressed.len() < content.len() {
                    serialize(
                        &compressed,
                        mime.as_ref(),
                        cache_control,
                        true,
                        Some("br"),
                        &etag,
                    )?
                } else {
                    // dont log robots.txt being smaller
                    if content.len() > 100 {
                        println!(
                            "Compressing {id:?} ({mime:?}) yielded a worse size! {} -> {}",
                            content.len(),
                            compressed.len()
                        );
                    }

                    identity.clone()
                }
            }
            None => identity.clone(),
        };

        let not_modified = serialize_304(&etag)?;

        Ok(Route {
            brotli,
            identity,
            not_modified,
            etag: etag.into_bytes().into(),
        })
    }
}

/// What quality of brotli compression we should do
/// `fast` will speed up this process for development
fn brotli_settings(mime: &Mime, fast: bool) -> Option<BrotliEncoderParams> {
    if fast {
        return None;
    }

    match mime.type_() {
        TEXT | APPLICATION => Some(BrotliEncoderParams {
            quality: 11,
            mode: BrotliEncoderMode::BROTLI_MODE_TEXT,
            ..Default::default()
        }),
        IMAGE if mime.subtype() == SVG => Some(BrotliEncoderParams {
            quality: 11,
            mode: BrotliEncoderMode::BROTLI_MODE_GENERIC,
            ..Default::default()
        }),
        _ => None,
    }
}

fn cache_control(mime: &Mime) -> Option<&'static str> {
    match mime.type_() {
        FONT => Some("public, max-age=31536000, immutable"),
        IMAGE => Some("public, max-age=86400"),
        _ => None,
    }
}

fn serialize(
    body: &[u8],
    content_type: &str,
    cache_control: Option<&str>,
    vary: bool,
    encoding: Option<&str>,
    etag: &str,
) -> Result<Box<[u8]>> {
    let mut buf = Vec::with_capacity(body.len() + 256);

    write!(buf, "HTTP/1.1 200 OK\r\n")?;
    write!(buf, "Content-Type: {content_type}\r\n")?;
    write!(buf, "Content-Length: {}\r\n", body.len())?;
    write!(buf, "ETag: {etag}\r\n")?;

    if let Some(enc) = encoding {
        write!(buf, "Content-Encoding: {enc}\r\n")?;
    }
    if let Some(cc) = cache_control {
        write!(buf, "Cache-Control: {cc}\r\n")?;
    }
    if vary {
        write!(buf, "Vary: Accept-Encoding\r\n")?;
    }

    buf.extend_from_slice(b"\r\n");
    buf.extend_from_slice(body);

    Ok(buf.into())
}

fn serialize_304(etag: &str) -> Result<Box<[u8]>> {
    let mut buf = Vec::with_capacity(64);

    write!(buf, "HTTP/1.1 304 Not Modified\r\n")?;
    write!(buf, "ETag: {etag}\r\n")?;
    write!(buf, "Content-Length: 0\r\n")?;

    buf.extend_from_slice(b"\r\n");

    Ok(buf.into())
}

fn compress_brotli(input: &[u8], mut params: BrotliEncoderParams) -> Vec<u8> {
    let mut buf = Vec::new();
    params.size_hint = input.len();
    BrotliCompress(&mut &input[..], &mut buf, &params).unwrap();
    buf
}
