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
            p { "© 2024 William (Liam) Snow IV" }
            div {
                a href="https://github.com/liamsnow/liamsnow.com" { "Source Code" }
                a href="https://github.com/liamsnow" { "GitHub" }
                a href="https://www.linkedin.com/in/william-snow-iv-140438169/" { "LinkedIn" }
                a href="mailto:mail@liamsnow.com" { "Email" }
                a href="https://raw.githubusercontent.com/LiamSnow/resume/refs/heads/main/resume.pdf" { "Resume" }
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

This is some `let more = "xa"`

Block code:
```rust
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
```


$$ \frac{1}{\sqrt{2}} $$

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam fringilla consequat lectus ac gravida. Aenean tincidunt quam efficitur dictum pulvinar. Nulla at malesuada neque, sed tristique mi. Fusce tempor sem pretium libero pharetra viverra. Donec sagittis nisl suscipit nisl porttitor, id dictum lorem fermentum. Praesent id turpis at magna facilisis tristique. Donec finibus lacinia est ac pulvinar. Nunc lacinia nisl leo. Sed vulputate viverra ipsum, ut molestie justo aliquet id. Suspendisse euismod non tellus et posuere. Sed eleifend vel urna eu consequat. Nunc fermentum lacus sem, sit amet feugiat felis elementum eu. Donec eu nulla ac massa semper tincidunt ut in mauris.

Sed sit amet congue elit. Morbi porta nec turpis ut faucibus. Sed eu ullamcorper ligula. Donec pellentesque in urna eu mattis. Phasellus rhoncus ornare neque quis feugiat. Fusce aliquet pellentesque leo nec laoreet. Mauris at rhoncus turpis. Morbi suscipit iaculis nisi, eu ornare turpis semper nec. Praesent quis ligula massa. Suspendisse quis aliquam nulla. Suspendisse pellentesque, nulla ut scelerisque porttitor, leo ipsum eleifend magna, ut mollis urna elit quis justo. Vestibulum dapibus, tellus placerat mollis dictum, purus turpis tincidunt sapien, et pulvinar eros purus at sem. Duis sit amet mi ligula. Vestibulum vel posuere tortor. Fusce scelerisque ante a lacinia rhoncus.

Suspendisse at arcu sed ligula tristique lobortis. Praesent at lacus dictum, pellentesque erat at, blandit nunc. Cras ut justo nulla. Praesent sed ex semper, sagittis velit a, tempor enim. Nulla volutpat nisi id tellus lobortis blandit. Curabitur in vulputate velit, ac pulvinar turpis. Pellentesque in est commodo, mollis purus ut, posuere magna. Donec non dui ac erat ornare posuere. Sed nec blandit felis. Aenean dignissim tincidunt augue, et efficitur risus pretium eget. Ut placerat accumsan pretium.

Etiam lorem dui, aliquam sit amet eros at, ullamcorper viverra nulla. Quisque lobortis ornare convallis. Sed suscipit elit eget tempus facilisis. Nulla ut pretium orci, vitae gravida ipsum. Morbi ac sapien at lacus cursus dapibus. Sed dignissim libero eu elementum tristique. Pellentesque finibus sapien id ipsum faucibus, at congue lectus congue. Fusce varius viverra lectus id faucibus. Sed auctor gravida orci, in bibendum orci tincidunt at. In hac habitasse platea dictumst. Vivamus eu nisl quis neque pretium sodales. Integer viverra aliquam tellus non aliquam. Proin ut arcu nulla.

Sed varius ultrices mauris quis ultrices. Maecenas semper nisi ut leo molestie interdum. In sit amet mauris at massa mattis venenatis. Mauris vel ante a est semper facilisis vitae id ipsum. Fusce ex enim, pellentesque a lorem gravida, convallis porta mi. Fusce eget hendrerit purus. In vel sollicitudin sem. Integer nec mattis nulla. In sed ipsum quis tortor consectetur mattis. Curabitur mollis neque vitae turpis pharetra aliquam. Vestibulum sed auctor enim, vitae efficitur sapien.
    "#,
        &Options::default(),
    ))
}

async fn blog() -> Markup {
    page(
        "Blog",
        html! {
            #content {
                (m())
            }
        },
        html! {
            link rel="stylesheet" href="/static/markdown.css";
        },
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
                    a href="/about" {
                        p { "About" }
                        p { "a" }
                    }
                    a href="/projects" {
                        p { "Projects" }
                        p { "p" }
                    }
                    a href="/blog" {
                        p { "Blog" }
                        p { "b" }
                    }
                    a href="/links" {
                        p { "Links" }
                        p { "l" }
                    }
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
