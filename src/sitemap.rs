use std::fmt::Write;

const BASE_URL: &str = "https://liamsnow.com";

pub fn generate(url_paths: &[String]) -> String {
    let mut xml = String::with_capacity(2048);
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push('\n');
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    xml.push('\n');

    for url_path in url_paths {
        let loc = if url_path == "/" {
            BASE_URL.to_string()
        } else {
            format!("{}{}", BASE_URL, url_path)
        };
        writeln!(xml, "  <url>").unwrap();
        writeln!(xml, "    <loc>{loc}</loc>").unwrap();
        writeln!(xml, "  </url>").unwrap();
    }

    xml.push_str("</urlset>");
    xml
}
