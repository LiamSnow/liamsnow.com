use crate::{RoutingTable, WatchArgs, web::route::Route};
use anyhow::Result;
use mime_guess::mime;
use std::fmt::Write;
use typst::syntax::{FileId, VirtualPath};

const SITEMAP_PATH: &str = "sitemap.xml";
const BASE_URL: &str = "https://liamsnow.com";

// TODO this can be run after indexing (might be more useful?)
pub fn generate(routes: &RoutingTable, watch: &WatchArgs) -> Result<(String, Route)> {
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

    let vp = VirtualPath::new("sitemap.xml");
    let id = FileId::new_fake(vp);
    let route = Route::compile(&id, xml.into(), &mime::TEXT_XML, watch.watch)?;
    Ok((SITEMAP_PATH.to_string(), route))
}
