use mime_guess::mime;
use rustc_hash::FxHashMap;
use std::fmt::Write;

use crate::compiler::Route;

const BASE_URL: &str = "https://liamsnow.com";

pub fn generate(routes: &FxHashMap<String, Route>) -> Route {
    let mut xml = String::with_capacity(2048);
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    xml.push('\n');

    for path in routes.keys() {
        if path.contains('.') {
            // skip files
            continue;
        }

        let loc = if path == "/" {
            BASE_URL.to_string()
        } else {
            format!("{}{}", BASE_URL, path)
        };
        writeln!(xml, "  <url>").unwrap();
        writeln!(xml, "    <loc>{loc}</loc>").unwrap();
        writeln!(xml, "  </url>").unwrap();
    }

    xml.push_str("</urlset>");
    Route::from_string(xml, mime::TEXT_XML)
}
