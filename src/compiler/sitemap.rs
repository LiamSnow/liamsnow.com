use crate::{RoutingTable, compiler::route::Route};
use mime_guess::mime;
use std::fmt::Write;

const SITEMAP_PATH: &str = "sitemap.xml";
const BASE_URL: &str = "https://liamsnow.com";

// TODO this can be run after indexing (might be more useful?)
pub fn generate(routes: &RoutingTable) -> (String, Route) {
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

    let route = Route::from_string(xml, mime::TEXT_XML, None);
    (SITEMAP_PATH.to_string(), route)
}

// TODO testing!!
