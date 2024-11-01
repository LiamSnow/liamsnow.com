use maud::{html, Markup};

use crate::page::make_page;

pub async fn get_home() -> Markup {
    make_page(
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
                    img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" style="box-shadow: 0px 1px 3px 1px #222";
                    img src="https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white" style="box-shadow: 0px 1px 3px 1px #00add8";
                    img src="https://img.shields.io/badge/c-%2300599C.svg?style=for-the-badge&logo=c&logoColor=white" style="box-shadow: 0px 1px 3px 1px #00599c";
                    img src="https://img.shields.io/badge/c%23-%23239120.svg?style=for-the-badge&logo=csharp&logoColor=white" style="box-shadow: 0px 1px 3px 1px #239120";
                    img src="https://img.shields.io/badge/swift-%23F15035.svg?style=for-the-badge&logo=swift&logoColor=white" style="box-shadow: 0px 1px 3px 1px #f15035";
                    img src="https://img.shields.io/badge/java-%23ED8B00.svg?style=for-the-badge&logo=openjdk&logoColor=white" style="box-shadow: 0px 1px 3px 1px #ed8b00";
                    img src="https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E" style="box-shadow: 0px 1px 3px 1px #323330";
                    img src="https://img.shields.io/badge/lua-%232C2D72.svg?style=for-the-badge&logo=lua&logoColor=white" style="box-shadow: 0px 1px 3px 1px #2c2d72";
                    img src="https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54" style="box-shadow: 0px 1px 3px 1px #3670A0";
                }
            }
        },
        html! {
            link rel="stylesheet" href="/static/home.css";
        },
    )
}
