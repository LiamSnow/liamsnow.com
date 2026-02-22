use crate::web::route::{BAD_REQUEST, NOT_FOUND, OK, UNAUTHORIZED};
use crate::{ROUTING_TABLE, WebArgs, update};
use anyhow::Result;
use httparse::{EMPTY_HEADER, Request, Status};
use memchr::memmem;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub mod route;

const MAX_HEADER_SIZE: usize = 16_384;
const MAX_BODY_SIZE: usize = 65_536;
const TIMEOUT: Duration = Duration::from_secs(5);
const BACKOFF: Duration = Duration::from_millis(10);
const HEADER_END: &[u8] = b"\r\n\r\n";
const UPDATE_PATH: &str = "/_update";

pub fn run(args: WebArgs, num_threads: usize) -> Result<()> {
    let addr = SocketAddr::new(args.address, args.port);
    let listener = TcpListener::bind(addr)?;

    println!("Hosting @ {addr} with {num_threads} threads");

    let listener = Arc::new(listener);

    for _ in 0..num_threads - 1 {
        let listener = listener.clone();
        thread::spawn(move || accept_loop(&listener));
    }

    accept_loop(&listener);
}

fn accept_loop(listener: &TcpListener) -> ! {
    loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                if let Err(e) = cfg_stream(&mut stream) {
                    eprintln!("Error configuring stream: {e}");
                }

                if let Err(e) = handle(&mut stream) {
                    eprintln!("Error handling stream: {e}");
                }
            }
            Err(e) => {
                eprintln!("accept error: {e}");
                thread::sleep(BACKOFF);
            }
        }
    }
}

fn cfg_stream(stream: &mut TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(TIMEOUT))?;
    stream.set_write_timeout(Some(TIMEOUT))?;
    stream.set_nodelay(true)
}

fn handle<S: Read + Write>(mut stream: S) -> io::Result<()> {
    let mut buf = [0u8; MAX_HEADER_SIZE];
    let mut filled = 0;

    loop {
        loop {
            if memmem::find(&buf[..filled], HEADER_END).is_some() {
                break;
            }

            if filled >= MAX_HEADER_SIZE {
                stream.write_all(BAD_REQUEST)?;
                return Ok(());
            }

            match stream.read(&mut buf[filled..]) {
                Ok(0) => return Ok(()),
                Ok(n) => filled += n,
                Err(_) => return Ok(()),
            }
        }

        let mut headers = [EMPTY_HEADER; 32];
        let mut req = Request::new(&mut headers);

        let body_offset = match req.parse(&buf[..filled]) {
            Ok(Status::Complete(n)) => n,
            _ => {
                stream.write_all(BAD_REQUEST)?;
                return Ok(());
            }
        };

        let method = req.method.unwrap_or("");
        let path = cut_query(req.path.unwrap_or("/"));
        let wants_close = connection_close(req.headers);

        match method {
            "GET" | "HEAD" => {
                handle_get(&mut stream, path, req.headers, method == "HEAD")?;
            }
            "POST" if path == UPDATE_PATH => {
                handle_webhook(&mut stream, req.headers, &buf[body_offset..filled])?;
                return Ok(());
            }
            "POST" => {
                stream.write_all(NOT_FOUND)?;
                return Ok(());
            }
            _ => {
                stream.write_all(route::METHOD_NOT_ALLOWED)?;
                return Ok(());
            }
        }

        if wants_close {
            return Ok(());
        }

        let rem = filled - body_offset;
        if rem > 0 {
            buf.copy_within(body_offset..filled, 0);
        }
        filled = rem;
    }
}

fn handle_get<S: Read + Write>(
    stream: &mut S,
    path: &str,
    headers: &[httparse::Header],
    head: bool,
) -> io::Result<()> {
    let table = ROUTING_TABLE.load();

    let Some(route) = table.get(path) else {
        return stream.write_all(NOT_FOUND);
    };

    let response = if etag_matches(headers, &route.etag) {
        route.not_modified.as_ref()
    } else if accepts_brotli(headers) {
        route.brotli.as_ref()
    } else {
        route.identity.as_ref()
    };

    if head && let Some(pos) = memmem::find(response, HEADER_END) {
        return stream.write_all(&response[..pos + 4]);
    }

    stream.write_all(response)
}

fn handle_webhook<S: Read + Write>(
    stream: &mut S,
    headers: &[httparse::Header],
    partial_body: &[u8],
) -> io::Result<()> {
    let Some(content_length) = find_header(headers, "content-length")
        .and_then(|val| str::from_utf8(val).ok())
        .and_then(|s| s.parse::<usize>().ok())
    else {
        stream.write_all(BAD_REQUEST)?;
        return Ok(());
    };

    if content_length == 0 {
        stream.write_all(BAD_REQUEST)?;
        return Ok(());
    }

    if content_length > MAX_BODY_SIZE {
        stream.write_all(route::PAYLOAD_TOO_LARGE)?;
        return Ok(());
    }

    let mut body = vec![0u8; content_length];
    let to_copy = partial_body.len().min(content_length);
    body[..to_copy].copy_from_slice(&partial_body[..to_copy]);

    let mut body_filled = to_copy;
    while body_filled < content_length {
        match stream.read(&mut body[body_filled..]) {
            Ok(0) => {
                stream.write_all(BAD_REQUEST)?;
                return Ok(());
            }
            Ok(n) => body_filled += n,
            Err(_) => return Ok(()),
        }
    }

    let Some(sig) = find_header(headers, update::GH_HEADER) else {
        stream.write_all(BAD_REQUEST)?;
        return Ok(());
    };

    if update::SECRET.get().is_none() {
        stream.write_all(NOT_FOUND)?;
        return Ok(());
    }

    if !update::verify(sig, body) {
        stream.write_all(UNAUTHORIZED)?;
        return Ok(());
    }

    stream.write_all(OK)?;
    stream.flush()?;

    thread::spawn(|| {
        if let Err(e) = update::run() {
            eprintln!("Update failed: {e}");
        }
    });

    Ok(())
}

fn accepts_brotli(headers: &[httparse::Header]) -> bool {
    find_header(headers, "accept-encoding").is_some_and(|v| v.windows(2).any(|w| w == b"br"))
}

fn find_header<'a>(headers: &[httparse::Header<'a>], name: &str) -> Option<&'a [u8]> {
    headers
        .iter()
        .find(|h| h.name.eq_ignore_ascii_case(name))
        .map(|h| h.value)
}

fn connection_close(headers: &[httparse::Header]) -> bool {
    find_header(headers, "connection").is_some_and(|v| v.eq_ignore_ascii_case(b"close"))
}

fn etag_matches(headers: &[httparse::Header], etag: &[u8]) -> bool {
    find_header(headers, "if-none-match")
        .is_some_and(|v| v == etag || v.windows(etag.len()).any(|w| w == etag))
}

fn cut_query(path: &str) -> &str {
    match memchr::memchr(b'?', path.as_bytes()) {
        Some(index) => &path[0..index],
        None => path,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{RoutingTable, web::route::Route};

    use super::*;

    #[test]
    fn test_cut_query() {
        assert_eq!(cut_query("/path"), "/path");
        assert_eq!(cut_query("path"), "path");
        assert_eq!(cut_query("cat/dog"), "cat/dog");
        assert_eq!(cut_query("cat/dog?ref=true"), "cat/dog");
        assert_eq!(cut_query("cat/dog/?ref=true"), "cat/dog/");
        assert_eq!(cut_query("cat/dog/test.css?ref=true"), "cat/dog/test.css");
        assert_eq!(cut_query(""), "");
        assert_eq!(cut_query("?"), "");
        assert_eq!(cut_query("?foo=bar"), "");
    }

    #[test]
    fn test_find_header() {
        let headers = [
            httparse::Header {
                name: "Content-Type",
                value: b"text/html",
            },
            httparse::Header {
                name: "Accept-Encoding",
                value: b"gzip, br",
            },
        ];
        assert_eq!(
            find_header(&headers, "content-type"),
            Some(b"text/html".as_slice())
        );
        assert_eq!(
            find_header(&headers, "Content-Type"),
            Some(b"text/html".as_slice())
        );
        assert_eq!(
            find_header(&headers, "CONTENT-TYPE"),
            Some(b"text/html".as_slice())
        );
        assert_eq!(find_header(&headers, "x-missing"), None);
    }

    #[test]
    fn test_find_header_empty() {
        let headers: [httparse::Header; 0] = [];
        assert_eq!(find_header(&headers, "anything"), None);
    }

    #[test]
    fn test_accepts_brotli() {
        let yes = [httparse::Header {
            name: "Accept-Encoding",
            value: b"gzip, br",
        }];
        let no = [httparse::Header {
            name: "Accept-Encoding",
            value: b"gzip, deflate",
        }];
        let only = [httparse::Header {
            name: "Accept-Encoding",
            value: b"br",
        }];
        let empty: [httparse::Header; 0] = [];

        assert!(accepts_brotli(&yes));
        assert!(!accepts_brotli(&no));
        assert!(accepts_brotli(&only));
        assert!(!accepts_brotli(&empty));
    }

    #[test]
    fn test_connection_close() {
        let close = [httparse::Header {
            name: "Connection",
            value: b"close",
        }];
        let keep = [httparse::Header {
            name: "Connection",
            value: b"keep-alive",
        }];
        let upper = [httparse::Header {
            name: "Connection",
            value: b"Close",
        }];
        let empty: [httparse::Header; 0] = [];

        assert!(connection_close(&close));
        assert!(!connection_close(&keep));
        assert!(connection_close(&upper));
        assert!(!connection_close(&empty));
    }

    #[test]
    fn test_etag_matches() {
        let etag = b"\"abc123\"";

        let exact = [httparse::Header {
            name: "If-None-Match",
            value: b"\"abc123\"",
        }];
        let multi = [httparse::Header {
            name: "If-None-Match",
            value: b"\"other\", \"abc123\"",
        }];
        let miss = [httparse::Header {
            name: "If-None-Match",
            value: b"\"xyz789\"",
        }];
        let empty: [httparse::Header; 0] = [];

        assert!(etag_matches(&exact, etag));
        assert!(etag_matches(&multi, etag));
        assert!(!etag_matches(&miss, etag));
        assert!(!etag_matches(&empty, etag));
    }

    struct MockStream {
        input: Cursor<Vec<u8>>,
        output: Vec<u8>,
    }

    impl MockStream {
        fn new(input: &[u8]) -> Self {
            Self {
                input: Cursor::new(input.to_vec()),
                output: Vec::new(),
            }
        }

        fn output(&self) -> &[u8] {
            &self.output
        }
    }

    impl Read for MockStream {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.input.read(buf)
        }
    }

    impl Write for MockStream {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn mock_routing_table() {
        let mut table = RoutingTable::default();
        table.insert(
            "/test".into(),
            Route {
                identity: b"HTTP/1.1 200 OK\r\nETag: \"t1\"\r\n\r\nidentity-body"
                    .to_vec()
                    .into_boxed_slice(),
                brotli:
                    b"HTTP/1.1 200 OK\r\nContent-Encoding: br\r\nETag: \"t1\"\r\n\r\nbrotli-body"
                        .to_vec()
                        .into_boxed_slice(),
                not_modified: b"HTTP/1.1 304 Not Modified\r\nETag: \"t1\"\r\n\r\n"
                    .to_vec()
                    .into_boxed_slice(),
                etag: b"\"t1\"".to_vec().into_boxed_slice(),
            },
        );
        ROUTING_TABLE.store(Arc::new(table));
    }

    #[test]
    fn get_path_identity() {
        mock_routing_table();
        let mut stream = MockStream::new(b"GET /test HTTP/1.1\r\nConnection: close\r\n\r\n");

        handle(&mut stream).unwrap();

        assert!(stream.output().starts_with(b"HTTP/1.1 200 OK"));
        assert!(stream.output().windows(13).any(|w| w == b"identity-body"));
    }

    #[test]
    fn get_path_brotli() {
        mock_routing_table();
        let mut stream = MockStream::new(
            b"GET /test HTTP/1.1\r\nAccept-Encoding: gzip, br\r\nConnection: close\r\n\r\n",
        );

        handle(&mut stream).unwrap();

        assert!(stream.output().windows(11).any(|w| w == b"brotli-body"));
    }

    #[test]
    fn get_invalid_path() {
        mock_routing_table();
        let mut stream = MockStream::new(b"GET /nope HTTP/1.1\r\nConnection: close\r\n\r\n");

        handle(&mut stream).unwrap();

        assert_eq!(stream.output(), NOT_FOUND);
    }

    #[test]
    fn test_head() {
        mock_routing_table();
        let mut stream = MockStream::new(b"HEAD /test HTTP/1.1\r\nConnection: close\r\n\r\n");

        handle(&mut stream).unwrap();

        let out = stream.output();
        assert!(out.starts_with(b"HTTP/1.1 200 OK"));
        assert!(memmem::find(out, HEADER_END).is_some());
        assert!(!out.windows(13).any(|w| w == b"identity-body"));
    }

    #[test]
    fn get_etag_hit() {
        mock_routing_table();
        let mut stream = MockStream::new(
            b"GET /test HTTP/1.1\r\nIf-None-Match: \"t1\"\r\nConnection: close\r\n\r\n",
        );

        handle(&mut stream).unwrap();

        assert!(stream.output().starts_with(b"HTTP/1.1 304"));
    }

    #[test]
    fn conn_close() {
        mock_routing_table();
        let mut stream = MockStream::new(
            b"GET /test HTTP/1.1\r\nConnection: close\r\n\r\n\
              GET /test HTTP/1.1\r\nConnection: close\r\n\r\n",
        );

        handle(&mut stream).unwrap();

        let count = stream
            .output()
            .windows(8)
            .filter(|w| *w == b"HTTP/1.1")
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn invalid_post() {
        mock_routing_table();
        let mut stream = MockStream::new(b"POST /test HTTP/1.1\r\nContent-Length: 0\r\n\r\n");

        handle(&mut stream).unwrap();

        assert_eq!(stream.output(), NOT_FOUND);
    }

    #[test]
    fn invalid_method() {
        mock_routing_table();
        let mut stream = MockStream::new(b"DELETE /test HTTP/1.1\r\nConnection: close\r\n\r\n");

        handle(&mut stream).unwrap();

        assert_eq!(stream.output(), route::METHOD_NOT_ALLOWED);
    }

    #[test]
    fn big_ass_header() {
        mock_routing_table();
        let garbage = vec![b'A'; MAX_HEADER_SIZE];
        let mut stream = MockStream::new(&garbage);

        handle(&mut stream).unwrap();

        assert_eq!(stream.output(), BAD_REQUEST);
    }

    #[test]
    fn bad_request() {
        mock_routing_table();
        let mut stream = MockStream::new(b"NOT A REAL REQUEST\r\n\r\n");

        handle(&mut stream).unwrap();

        assert_eq!(stream.output(), BAD_REQUEST);
    }

    #[test]
    fn keepalive() {
        mock_routing_table();
        let mut stream = MockStream::new(
            b"GET /test HTTP/1.1\r\n\r\n\
              GET /test HTTP/1.1\r\nConnection: close\r\n\r\n",
        );

        handle(&mut stream).unwrap();

        let count = stream
            .output()
            .windows(8)
            .filter(|w| *w == b"HTTP/1.1")
            .count();
        assert_eq!(count, 2);
    }
}
