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

#[cfg(test)]
mod tests {
    use super::*;
    use brotli::BrotliDecompress;
    use httparse::{EMPTY_HEADER, Header, Response, Status};
    use mime_guess::mime;
    use typst::syntax::VirtualPath;

    #[test]
    fn test_cache_control() {
        assert!(cache_control(&mime::IMAGE_PNG).is_some());
        assert!(cache_control(&mime::TEXT_HTML).is_none());
        assert!(cache_control(&mime::APPLICATION_JSON).is_none());
        let cc = cache_control(&mime::FONT_WOFF2).unwrap();
        assert!(cc.contains("immutable"));
    }

    #[test]
    fn test_brotli_settings() {
        assert!(brotli_settings(&mime::TEXT_HTML, false).is_some());
        assert!(brotli_settings(&mime::APPLICATION_JSON, false).is_some());
        assert!(brotli_settings(&mime::IMAGE_SVG, false).is_some());
        assert!(brotli_settings(&mime::IMAGE_PNG, false).is_none());
        assert!(brotli_settings(&mime::IMAGE_JPEG, false).is_none());
        assert!(brotli_settings(&mime::FONT_WOFF2, false).is_none());
    }

    #[test]
    fn test_compress_brotli() {
        let input = b"According to all known laws of aviation, there is no way a bee should be able to fly.";
        let params = BrotliEncoderParams {
            quality: 4,
            mode: BrotliEncoderMode::BROTLI_MODE_TEXT,
            ..Default::default()
        };

        let compressed = compress_brotli(input, params);
        let mut decompressed = Vec::new();
        BrotliDecompress(&mut &compressed[..], &mut decompressed).unwrap();
        assert_eq!(decompressed, input);
    }

    fn parse_response<'a>(
        raw: &'a [u8],
        headers: &'a mut [Header<'a>],
    ) -> (Response<'a, 'a>, &'a [u8]) {
        let mut resp = Response::new(headers);
        let body_offset = match resp.parse(raw) {
            Ok(Status::Complete(n)) => n,
            other => panic!("Failed to parse response: {other:?}"),
        };
        (resp, &raw[body_offset..])
    }

    fn find_header<'a>(headers: &[Header<'a>], name: &str) -> Option<&'a [u8]> {
        headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| h.value)
    }

    fn assert_header(headers: &[Header], name: &str, expected: &[u8]) {
        let val = find_header(headers, name).unwrap_or_else(|| panic!("missing header: {name}"));
        assert_eq!(val, expected, "header {name} mismatch");
    }

    fn assert_no_header(headers: &[Header], name: &str) {
        assert!(
            find_header(headers, name).is_none(),
            "header {name} should be absent"
        );
    }

    #[test]
    fn static_responses_valid_http() {
        for (name, raw, expected_code) in [
            ("BAD_REQUEST", BAD_REQUEST, 400),
            ("NOT_FOUND", NOT_FOUND, 404),
            ("METHOD_NOT_ALLOWED", METHOD_NOT_ALLOWED, 405),
            ("PAYLOAD_TOO_LARGE", PAYLOAD_TOO_LARGE, 413),
            ("OK", OK, 200),
            ("UNAUTHORIZED", UNAUTHORIZED, 401),
        ] {
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let (resp, body) = parse_response(raw, &mut headers);

            assert_eq!(
                resp.code.unwrap(),
                expected_code,
                "{name} wrong status code"
            );
            assert_header(resp.headers, "Content-Length", b"0");
            assert_header(resp.headers, "Connection", b"close");
            assert!(body.is_empty(), "{name} should have empty body");
        }
    }

    #[test]
    fn serialize_minimal() {
        let raw = serialize(b"hello", "text/plain", None, false, None, "\"e1\"").unwrap();
        let mut headers = [EMPTY_HEADER; 16];
        let (resp, body) = parse_response(&raw, &mut headers);

        assert_eq!(resp.code.unwrap(), 200);
        assert_header(resp.headers, "Content-Type", b"text/plain");
        assert_header(resp.headers, "Content-Length", b"5");
        assert_header(resp.headers, "ETag", b"\"e1\"");
        assert_no_header(resp.headers, "Content-Encoding");
        assert_no_header(resp.headers, "Cache-Control");
        assert_no_header(resp.headers, "Vary");
        assert_eq!(body, b"hello");
    }

    #[test]
    fn serialize_all_optional_headers() {
        let raw = serialize(
            b"data",
            "text/html",
            Some("public, max-age=86400"),
            true,
            Some("br"),
            "\"e2\"",
        )
        .unwrap();
        let mut headers = [EMPTY_HEADER; 16];
        let (resp, body) = parse_response(&raw, &mut headers);

        assert_eq!(resp.code.unwrap(), 200);
        assert_header(resp.headers, "Content-Encoding", b"br");
        assert_header(resp.headers, "Cache-Control", b"public, max-age=86400");
        assert_header(resp.headers, "Vary", b"Accept-Encoding");
        assert_eq!(body, b"data");
    }

    #[test]
    fn serialize_empty_body() {
        let raw = serialize(b"", "text/plain", None, false, None, "\"e0\"").unwrap();
        let mut headers = [EMPTY_HEADER; 16];
        let (resp, body) = parse_response(&raw, &mut headers);

        assert_eq!(resp.code.unwrap(), 200);
        assert_header(resp.headers, "Content-Length", b"0");
        assert!(body.is_empty());
    }

    #[test]
    fn serialize_304_structure() {
        let raw = serialize_304("\"abc\"").unwrap();
        let mut headers = [EMPTY_HEADER; 16];
        let (resp, body) = parse_response(&raw, &mut headers);

        assert_eq!(resp.code.unwrap(), 304);
        assert_header(resp.headers, "ETag", b"\"abc\"");
        assert_header(resp.headers, "Content-Length", b"0");
        assert!(body.is_empty());
    }

    fn test_file_id(name: &str) -> FileId {
        FileId::new(None, VirtualPath::new(name))
    }

    fn compressible_body() -> Vec<u8> {
        "<html><body>".repeat(200).into_bytes()
    }

    #[test]
    fn compile_html_produces_brotli() {
        let route = Route::compile(
            &test_file_id("t.html"),
            compressible_body(),
            &mime::TEXT_HTML,
            false,
        )
        .unwrap();

        assert_ne!(route.identity, route.brotli);

        let mut headers = [EMPTY_HEADER; 16];
        let (resp, _) = parse_response(&route.brotli, &mut headers);
        assert_header(resp.headers, "Content-Encoding", b"br");
    }

    #[test]
    fn compile_png_no_brotli() {
        let content = vec![0u8; 200];
        let route =
            Route::compile(&test_file_id("t.png"), content, &mime::IMAGE_PNG, false).unwrap();

        assert_eq!(route.identity, route.brotli);
    }

    #[test]
    fn compile_etag_deterministic() {
        let body = b"stable content".to_vec();
        let r1 = Route::compile(&test_file_id("a"), body.clone(), &mime::TEXT_HTML, true).unwrap();
        let r2 = Route::compile(&test_file_id("b"), body, &mime::TEXT_HTML, true).unwrap();

        assert_eq!(r1.etag, r2.etag);
    }

    #[test]
    fn compile_etag_differs() {
        let r1 =
            Route::compile(&test_file_id("a"), b"aaa".to_vec(), &mime::TEXT_HTML, true).unwrap();
        let r2 =
            Route::compile(&test_file_id("a"), b"bbb".to_vec(), &mime::TEXT_HTML, true).unwrap();

        assert_ne!(r1.etag, r2.etag);
    }

    #[test]
    fn compile_304_contains_etag() {
        let route = Route::compile(
            &test_file_id("t.html"),
            b"x".to_vec(),
            &mime::TEXT_HTML,
            true,
        )
        .unwrap();

        let mut headers = [EMPTY_HEADER; 16];
        let (resp, _) = parse_response(&route.not_modified, &mut headers);

        assert_eq!(resp.code.unwrap(), 304);
        assert_header(resp.headers, "ETag", &route.etag);
    }
}
