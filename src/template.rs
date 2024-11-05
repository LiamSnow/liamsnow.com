use std::fs;

use maud::{html, Markup, PreEscaped, DOCTYPE};

pub const CSS_MAIN: &str = "main";
pub const CSS_HOME: &str = "home";
pub const CSS_INDEX: &str = "index";
pub const CSS_POST: &str = "post";

pub fn apply(url: &str, title: &str, content: Markup, css: &[&str]) -> Markup {
    html! {
        (DOCTYPE)
        html lang = "en" {
            head {
                meta charset="utf-8";
                title { (title) }
                (inject_css(css))
            }
            body {
                main {
                    #content {
                        @if url == "/" {
                            .mark .top .left { "+" }
                        }
                        @else {
                            a .mark .top .back href=(get_back_link(url)) { "〈〈〈" }
                            .mark .url { (url) }
                        }

                        .mark .top .right { "+" }
                        .mark .bottom .left { "+" }
                        .mark .bottom .right { "+" }

                        #box {
                            (content)
                        }
                    }
                }

                (footer())
            }
        }
    }
}

fn get_back_link(url: &str) -> &str {
    match url.rfind('/') {
        Some(last_index) if last_index > 0 => &url[..last_index],
        _ => "/",
    }
}

fn footer() -> Markup {
    html! {
        footer {
            div {
                p { "© 2024 William (Liam) Snow IV" }
                a href="https://github.com/liamsnow/liamsnow.com" { "(Source Code)" }
            }
            div {
              p { "Made with 🦀" }
            }
            div {
                a href="mailto:mail@liamsnow.com" { "Email" }
                a href="https://www.linkedin.com/in/william-snow-iv-140438169/" { "LinkedIn" }
                a href="https://github.com/liamsnow" { "GitHub" }
            }
        }
    }
}

#[cfg(feature = "dev")]
fn inject_css(files: &[&str]) -> Markup {
    html! {
        @for file in files {
            link rel="stylesheet" href=(format!("/static/{file}.css"));
        }
    }
}

#[cfg(not(feature = "dev"))]
fn inject_css(files: &[&str]) -> Markup {
    let mut s = String::new();
    for file in files {
        let path = format!("./static/{file}.css");
        let contents = fs::read_to_string(path).expect(&format!("/* ERROR FAILED TO READ {file} */"));
        s += &(contents + "\n\n");
    }
    html! {
        style {
            (PreEscaped(s))
        }
    }
}
