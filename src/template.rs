use maud::{DOCTYPE, Markup, PreEscaped, html};
use minify_js::{Session, TopLevelMode, minify};
use std::{fs, sync::OnceLock};

#[cfg(not(feature = "dev"))]
use crate::scss;

static PRELOAD_JS: OnceLock<PreEscaped<String>> = OnceLock::new();

const SCHEMA: &str = r#"{
    "@context": "https://schema.org",
    "@type": "WebSite",
    "name": "Liam Snow",
    "url": "https://liamsnow.com",
}"#;

pub fn apply(
    path: &str,
    title: &str,
    desc: &str,
    scss: &str,
    header_content: Markup,
    content: Markup,
    jsonld: Option<&str>,
) -> Markup {
    let canonical_url = format!("https://liamsnow.com{path}");

    html! {
        (DOCTYPE)
        html lang = "en" {
            head {
                title { (title) }
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content=(desc);
                meta name="author" content="William Snow IV";
                link rel="canonical" href=(canonical_url);

                meta property="og:title" content=(title);
                meta property="og:description" content=(desc);
                meta property="og:type" content="website";
                meta property="og:url" content=(canonical_url);
                meta property="og:site_name" content="Liam Snow";

                meta name="twitter:card" content="summary";
                meta name="twitter:title" content=(title);
                meta name="twitter:description" content=(desc);

                link rel="alternate" type="application/rss+xml" title="Liam Snow's Blog" href="/blog/rss.xml";
                link rel="alternate" type="application/rss+xml" title="Liam Snow's Projects" href="/projects/rss.xml";

                link rel="preload" href="/static/fonts/SpaceMono-Regular.ttf" as="font" type="font/ttf" crossorigin="anonymous";
                link rel="preload" href="/static/fonts/SpaceMono-Bold.ttf" as="font" type="font/ttf" crossorigin="anonymous";
                link rel="preload" href="/static/fonts/SpaceGrotesk-Regular.otf" as="font" type="font/otf" crossorigin="anonymous";

                (inject_scss(scss))
                (header_content)

                script type="application/ld+json" {
                    (PreEscaped(SCHEMA))
                }

                @if let Some(json_ld) = jsonld {
                    script type="application/ld+json" {
                        (PreEscaped(json_ld))
                    }
                }
            }
            body {
                (header(path))

                main {
                    #content {
                        (content)
                    }
                }

                (footer())

                script {
                    (get_preload_js())
                }
            }
        }
    }
}

fn get_base_url(path: &str) -> String {
    if path == "/" {
        return "".to_string();
    }
    let num_slashes = path.chars().filter(|c| *c == '/').count();
    "../".repeat(num_slashes)
}

fn header(path: &str) -> Markup {
    html! {
        header {
            .container {
                .left {
                    (link("IV", &get_base_url(path)))
                }
                .nav.desktop {
                    (link("BLOG", "/blog"))
                    (link("PROJECTS", "/projects"))
                }
                .nav.mobile {
                    button { "MENU" }
                }
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            .container {
                .left {
                    (link_new_tab("EMAIL", "mailto:mail@liamsnow.com"))
                    (link_new_tab("LINKEDIN", "https://www.linkedin.com/in/william-snow-iv-140438169/"))
                    (link_new_tab("GITHUB", "https://github.com/liamsnow"))
                    (link_new_tab("RESUME", "https://github.com/LiamSnow/resume/blob/main/resume.pdf"))
                }
                p.right {
                    "© 2025 William Snow IV"
                    br;
                    (link_new_tab("Made with Rust 🦀", "https://github.com/liamsnow/liamsnow.com"))
                }
            }
        }
    }
}

pub fn link(text: &str, href: &str) -> Markup {
    html! {
        a href=(href) { (text) }
    }
}

pub fn link_new_tab(text: &str, href: &str) -> Markup {
    html! {
        a target="_blank" href=(href) { (text) }
    }
}

#[cfg(feature = "dev")]
fn inject_scss(name: &str) -> Markup {
    html! {
        link rel="stylesheet" href=(format!("/static/dist/{name}.css"));
    }
}

#[cfg(not(feature = "dev"))]
fn inject_scss(name: &str) -> Markup {
    let css = scss::compile_file(name);
    html! {
        style {
            (PreEscaped(css))
        }
    }
}

pub fn load_js(name: &str) -> PreEscaped<String> {
    let file_path = format!("./static/{name}.js");
    let code = fs::read(&file_path).expect("Failed to read JavaScript file");
    let session = Session::new();
    let mut output = Vec::new();
    minify(&session, TopLevelMode::Global, &code, &mut output).expect("minify-js failed");
    PreEscaped(String::from_utf8(output).expect("in minify-js output"))
}

pub fn get_preload_js() -> &'static PreEscaped<String> {
    PRELOAD_JS.get_or_init(|| load_js("preload"))
}
