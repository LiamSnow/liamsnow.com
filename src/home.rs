use std::sync::OnceLock;

use maud::{html, Markup};

use crate::template;

static HOME_HTML: OnceLock<Markup> = OnceLock::new();

pub fn init() {
    HOME_HTML.get_or_init(|| make_home_html());
}

pub async fn get_home() -> Markup {
    HOME_HTML.get().unwrap().clone()
}

pub fn make_home_html() -> Markup {
    template::apply(
        "/",
        "Home",
        html! {
            #name {
                h1 { "LIAM SNOW" }
            }
            #about {
                p { "I am a passionate programmer who XXXX XXXX XXXX XXX." }
                ul {
                    li {
                        "BS/MS CS student at "
                        a href="https://www.wpi.edu/" { "WPI" }
                    }
                    li {
                        "Email me at "
                        a href="mailto:mail@liamsnow.com" { "mail@liamsnow.com" }
                    }
                    li {
                        "Download my "
                        a href="https://raw.githubusercontent.com/LiamSnow/resume/refs/heads/main/resume.pdf" { "resume" }
                    }
                }

            }
            #links {
                a href="/blog" {
                    p { "Blog" }
                }
                a href="/projects" {
                    p { "Projects" }
                }
                a href="https://github.com/liamsnow" {
                    p .outbound { "GitHub" }
                }
                a href="https://www.linkedin.com/in/william-snow-iv-140438169/" {
                    p .outbound { "LinkedIn" }
                }
            }
            #langs-title { "Languages" }
            #langs-container {
                #langs {
                    img src="/static/langs/rust.svg" style="box-shadow: 0px 1px 3px 1px #222";
                    img src="/static/langs/go.svg" style="box-shadow: 0px 1px 3px 1px #00add8";
                    img src="/static/langs/c.svg" style="box-shadow: 0px 1px 3px 1px #00599c";
                    img src="/static/langs/csharp.svg" style="box-shadow: 0px 1px 3px 1px #239120";
                    img src="/static/langs/swift.svg" style="box-shadow: 0px 1px 3px 1px #f15035";
                    img src="/static/langs/java.svg" style="box-shadow: 0px 1px 3px 1px #ed8b00";
                    img src="/static/langs/javascript.svg" style="box-shadow: 0px 1px 3px 1px #323330";
                    img src="/static/langs/lua.svg" style="box-shadow: 0px 1px 3px 1px #2c2d72";
                    img src="/static/langs/python.svg" style="box-shadow: 0px 1px 3px 1px #3670A0";
                }
            }
        },
        &[template::CSS_MAIN, template::CSS_HOME]
    )
}

