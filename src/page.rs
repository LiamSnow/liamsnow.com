use maud::{html, Markup, DOCTYPE};

pub fn footer() -> Markup {
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

const DEFAULT_CSS: &str = "/static/main.css";

// TODO take list of CSS file names, and inject
pub fn make_page(url: &str, title: &str, content: Markup, head: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang = "en" {
            head {
                meta charset="utf-8";
                title { (title) }
                link rel="stylesheet" href="/static/main.css";
                (head)
            }
            body {
                main {
                    #content {
                        @if url == "/" {
                            .mark .top .left { "+" }
                        }
                        @else {
                            a .mark .top .back href="../" { "〈〈〈" }
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
