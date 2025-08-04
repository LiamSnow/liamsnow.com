use maud::{DOCTYPE, Markup, html};

pub const CSS_MAIN: &str = "main";
pub const CSS_HOME: &str = "home";
pub const CSS_INDEX: &str = "index";
pub const CSS_POST: &str = "post";
pub const KATEX: &str = "katex";

pub fn apply(
    path: &str,
    title: &str,
    content: Markup,
    css: &[&str],
    js: &[&str],
    include_katex: bool,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang = "en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                (inject_css(css))
                @if include_katex {
                    link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.css" integrity="sha384-5TcZemv2l/9On385z///+d7MSYlvIEw9FuZTIdZ14vJLqWphw7e7ZPuOiCHJcFCP" crossorigin="anonymous";
                    script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.js" integrity="sha384-cMkvdD8LoxVzGF/RPUKAcvmm49FQ0oxwDF3BGKtDXcEc+T1b2N+teh/OJfpU0jr6" crossorigin="anonymous" {}
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
                (inject_js(js))
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
                    a href=(get_base_url(path)) { "IV" }
                }
                .nav.desktop {
                    a href="/blog" { "BLOG" }
                    a href="/projects" { "PROJECTS" }
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
                    a target="_blank" href="mailto:mail@liamsnow.com" { "EMAIL" }
                    a target="_blank" href="https://www.linkedin.com/in/william-snow-iv-140438169/" { "LINKEDIN" }
                    a target="_blank" href="https://github.com/liamsnow" { "GITHUB" }
                    a target="_blank" href="https://github.com/LiamSnow/resume/blob/main/resume.pdf" { "RESUME" }
                }
                p.right {
                    "© 2025 William Snow IV"
                    br;
                    "Made with Rust 🦀"
                }
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

#[cfg(feature = "dev")]
fn inject_js(files: &[&str]) -> Markup {
    html! {
        @for file in files {
            script type="text/javascript" src=(format!("/static/{file}.js")) {}
        }
    }
}

#[cfg(not(feature = "dev"))]
fn inject_css(files: &[&str]) -> Markup {
    let mut s = String::new();
    for file in files {
        let path = format!("./static/{file}.css");
        let contents = fs::read_to_string(path).expect(&format!("ERROR FAILED TO READ {file}.css"));
        s += &(contents + "\n\n");
    }
    html! {
        style {
            (PreEscaped(s))
        }
    }
}

#[cfg(not(feature = "dev"))]
fn inject_js(files: &[&str]) -> Markup {
    let mut s = String::new();
    for file in files {
        let path = format!("./static/{file}.js");
        let contents = fs::read_to_string(path).expect(&format!("ERROR FAILED TO READ {file}.js"));
        s += &(contents + "\n\n");
    }
    html! {
        script {
            (PreEscaped(s))
        }
    }
}
