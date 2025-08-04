use crate::post::PostCollection;
use jiff::civil::DateTime;
use std::fmt::Write;

const BASE_URL: &str = "https://liamsnow.com";
const PRIORITY_HOME: &str = "1.0";
const PRIORITY_INDEX: &str = "0.8";
const PRIORITY_POST: &str = "0.6";

pub fn generate(blogs: &PostCollection, projects: &PostCollection) -> String {
    let mut xml = String::with_capacity(2048);
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>\n"#);
    xml.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n"#);

    add_url(&mut xml, BASE_URL, None, PRIORITY_HOME);

    add_collection_urls(&mut xml, "blog", blogs);
    add_collection_urls(&mut xml, "projects", projects);

    xml.push_str("</urlset>");
    xml
}

fn add_collection_urls(xml: &mut String, collection: &str, posts: &PostCollection) {
    add_url(
        xml,
        &format!("{BASE_URL}/{collection}"),
        None,
        PRIORITY_INDEX,
    );

    for (key, post) in &posts.posts {
        add_url(
            xml,
            &format!("{BASE_URL}/{collection}/{key}"),
            Some(&post.meta.date),
            PRIORITY_POST,
        );
    }
}

fn add_url(xml: &mut String, loc: &str, lastmod: Option<&DateTime>, priority: &str) {
    writeln!(xml, "  <url>").unwrap();
    writeln!(xml, "    <loc>{loc}</loc>").unwrap();

    if let Some(date) = lastmod {
        writeln!(xml, "    <lastmod>{}</lastmod>", date.strftime("%Y-%m-%d")).unwrap();
    }

    writeln!(xml, "    <priority>{priority}</priority>").unwrap();
    writeln!(xml, "  </url>").unwrap();
}
