use axum::{
    routing::{get, get_service},
    Router,
};
use comrak::{markdown_to_html, Options};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tower_http::services::{ServeDir, ServeFile};

fn footer() -> Markup {
    html! {
        footer {
            p { "© 2024 Liam Snow" }
            div {
                a href="https://github.com/liamsnow/liamsnow.com" { "Source Code" }
                a href="https://github.com/liamsnow" { "GitHub" }
                a href="" { "LinkedIn" }
                a href="" { "Email" }
                a href="" { "Resume" }
            }
        }
    }
}

fn page(title: &str, content: Markup, head: Markup) -> Markup {
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
                (content)

                (footer())
            }
        }
    }
}

fn m() -> Markup {
    PreEscaped(markdown_to_html(
        r#"
# H1
## H2
### H3
#### H4

This is __bold__

and this _italics_!

List:
1. asdf
2. asdf
3. asdf

Inline `code`.

Block code:
```rust
let m = 2;
```


$$ \frac{1}{\sqrt{2}} $$
    "#,
        &Options::default(),
    ))
}

async fn blog() -> Markup {
    page(
        "Blog",
        html! {
            #content {
                // (m())
                h1 {"example"}
            }
        },
        html! {},
    )
}

async fn home() -> Markup {
    page(
        "Home",
        html! {
            #content {
                #name {
                    pre { "██╗     ██╗ █████╗ ███╗   ███╗    ███████╗███╗   ██╗ ██████╗ ██╗    ██╗" }
                    pre { "██║     ██║██╔══██╗████╗ ████║    ██╔════╝████╗  ██║██╔═══██╗██║    ██║" }
                    pre { "██║     ██║███████║██╔████╔██║    ███████╗██╔██╗ ██║██║   ██║██║ █╗ ██║" }
                    pre { "██║     ██║██╔══██║██║╚██╔╝██║    ╚════██║██║╚██╗██║██║   ██║██║███╗██║" }
                    pre { "███████╗██║██║  ██║██║ ╚═╝ ██║    ███████║██║ ╚████║╚██████╔╝╚███╔███╔╝" }
                    pre { "╚══════╝╚═╝╚═╝  ╚═╝╚═╝     ╚═╝    ╚══════╝╚═╝  ╚═══╝ ╚═════╝  ╚══╝╚══╝ " }
                }
                p #caption { "This website was made using 🦀  and ❤️" }
                #links {
                    a href="/about" { "About" }
                    a href="/projects" { "Projects" }
                    a href="/blog" { "Blog" }
                    a href="/links" { "Links" }
                }
            }
        },
        html! {
            link rel="stylesheet" href="/static/home.css";
        },
    )
}

async fn handle_404() -> Markup {
    html! {
        h1 { "404" }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/blog", get(blog))
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./static/robots.txt")),
        )
        .route(
            "/favicon.ico",
            get_service(ServeFile::new("./static/favicon.ico")),
        )
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handle_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
