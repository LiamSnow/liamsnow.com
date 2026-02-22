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
/// only for github webhook
const MAX_BODY_SIZE: usize = 65_536;
const TIMEOUT: Duration = Duration::from_secs(5);
const BACKOFF: Duration = Duration::from_millis(10);
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
                _ = handle(&mut stream);
            }
            Err(e) => {
                eprintln!("accept error: {e}");
                thread::sleep(BACKOFF);
            }
        }
    }
}

fn handle(stream: &mut TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(TIMEOUT))?;
    stream.set_write_timeout(Some(TIMEOUT))?;
    stream.set_nodelay(true)?;

    let mut buf = [0u8; MAX_HEADER_SIZE];
    let mut filled = 0;

    loop {
        loop {
            match stream.read(&mut buf[filled..]) {
                Ok(0) => return Ok(()),
                Ok(n) => filled += n,
                Err(_) => return Ok(()),
            }

            if memmem::find(&buf[..filled], b"\r\n\r\n").is_some() {
                break;
            }

            if filled >= MAX_HEADER_SIZE {
                stream.write_all(BAD_REQUEST)?;
                return Ok(());
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
        let path = req.path.unwrap_or("/");
        let wants_close = connection_close(req.headers);

        match method {
            "GET" | "HEAD" => {
                handle_get(stream, path, req.headers, method == "HEAD")?;
            }
            "POST" if path == UPDATE_PATH => {
                handle_update(stream, req.headers, &buf[body_offset..filled])?;
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

fn handle_get(
    stream: &mut TcpStream,
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

    if head && let Some(pos) = memmem::find(response, b"\r\n\r\n") {
        return stream.write_all(&response[..pos + 4]);
    }

    stream.write_all(response)
}

fn handle_update(
    stream: &mut TcpStream,
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
