use maud::{html, Markup, DOCTYPE};


pub fn footer() -> Markup {
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

pub fn make_page(title: &str, content: Markup, head: Markup) -> Markup {
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
