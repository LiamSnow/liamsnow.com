use std::sync::OnceLock;

use maud::{Markup, html};

use crate::{
    post::{PostMeta, format_date},
    template,
};

static HOME_HTML: OnceLock<Markup> = OnceLock::new();

pub fn init(recent_projects: Vec<(String, PostMeta)>, recent_blogs: Vec<(String, PostMeta)>) {
    HOME_HTML.get_or_init(|| make_home_html(recent_projects, recent_blogs));
}

pub async fn get_home() -> Markup {
    HOME_HTML.get().unwrap().clone()
}

const ABOUT_ME: &str = "
I'm a Computer Science MS and Electrical & Computer Engineering BS student at
Worcester Polytechnic Institute with a passion for building efficient, reliable systems.
I enjoy learning in all areas of CS and have experience in many languages:
Rust, Golang, C, C++, Python, Java, C#, TypeScript, JavaScript, Lua, Swift, and others.
However, my passion truly lies in writing Rust 🦀.
";


pub fn make_home_html(
    recent_projects: Vec<(String, PostMeta)>,
    recent_blogs: Vec<(String, PostMeta)>,
) -> Markup {
    template::apply(
        "/",
        "Home",
        html! {
            #hero {
                h1.desktop {
                    "LIAMSNOW"
                }

                h1.mobile{
                    "LIAM"
                    br;
                    "SNOW"
                }

                .content {
                    .left {}
                    .right {
                        h2 { "About me" }
                        p { (ABOUT_ME) }
                        a href="mailto:mail@liamsnow.com" { "EMAIL" }
                        a target="_blank" href="https://www.linkedin.com/in/william-snow-iv-140438169/" { "LINKEDIN" }
                        a target="_blank" href="https://github.com/liamsnow" { "GITHUB" }
                        a target="_blank" href="https://github.com/LiamSnow/resume/blob/main/resume.pdf" { "RESUME" }
                    }
                }
            }

            #sections {
                section {
                    .header {
                        span { "/ SKILLS" }
                        span { "■" }
                    }
                    .grid {
                        div {
                            h3 { "Systems Programming" }
                            p {
                                "I love writing fast, low-level code code, and especially working with binary protocols.
                                Recently I have written the ESPHome protocol in rust ("
                                a href="/projects/esphomebridge-rs" { "esphomebridge-rs" }
                                ") and also made open source software for the Eightsleep ("
                                a href="/projects/opensleep" { "opensleep" }
                                ") which fully implements the undocumented protocols for temperature control and sleep tracking."
                            }
                        }
                        div {
                            h3 { "Backend/API" }
                            p {
                                "I've been writing backends and APIs from around the time
                                I started programming. I've made them in many languages
                                Rust (Axum, Actix), Go (Gin), .NET, FastAPI to name
                                a few.
                                I've worked with REST and WebSockets extensively and have
                                some GraphQL experience.
                                For databases, I've worked in many things too: SQL (PostgreSQL, SQLite, Amazon RDS) and NoSQL (GC Datastore, SledDB)"
                            }
                        }
                        div {
                            h3 { "Architecture" }
                            p {
                                "My approach to writing software starts with thoughfully
                                planning out architecture, and evolving that architecture
                                early on. I believe that this is the foundation of good software. When systems are well planned, everything else fall into place, bugs are easier to fix and harder to make in the first place. Furthermore, it becomes easier to add new features and work on with many engineers."
                            }
                        }
                    }
                }

                section {
                    .header {
                        span { "/ PROJECTS" }
                        span { "■" }
                    }
                    .lined-grid {
                        @for (key, meta) in &recent_projects {
                           a href=(format!("/projects/{key}")) {
                               h3 { (meta.title) }
                               p.desc { (meta.desc) }
                               p.date { (format_date(&meta.date)) }
                           }
                        }
                    }
                }

                section {
                    .header {
                        span { "/ BLOG" }
                        span { "■" }
                    }
                    .lined-grid {
                        @for (key, meta) in &recent_blogs {
                           a href=(format!("/blog/{key}")) {
                               h3 { (meta.title) }
                               p.desc { (meta.desc) }
                               p.date { (format_date(&meta.date)) }
                           }
                        }
                    }
                }
            }
        },
        &[template::CSS_MAIN, template::CSS_HOME],
        &[],
        false,
    )
}
