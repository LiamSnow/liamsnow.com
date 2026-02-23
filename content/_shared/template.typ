#let page = sys.inputs.at("page", default: (:))

#let link(text, href) = {
  html.a(href: href)[#text]
}

#let link-new-tab(text, href) = {
  html.a(target: "_blank", href: href)[#text]
}

#let link-new-tab-highlight(text, href) = {
  html.a(target: "_blank", class: "highlight", href: href)[#text]
}

#let header() = {
  html.header[
    #html.div(class: "container")[
      #html.div(class: "left")[
        #link("IV", "/")
      ]
      #html.nav(class: "nav")[
        #html.button(class: "light-dark", aria-label: "Toggle dark mode")[
          #html.span(class: "moon", style: "display: none")[
            #html.img(
              src: "/icons/moon.svg",
              alt: "Enable dark mode icon",
              width: 26,
              height: 26
            )
          ]
          #html.span(class: "sun", style: "display: none")[
            #html.img(
              src: "/icons/sun.svg",
              alt: "Enable light mode icon",
              width: 26,
              height: 26
            )
          ]
        ]
        #link("BLOG", "/blog")
        #link("PROJECTS", "/projects")
      ]
    ]
  ]
}

#let social() = {
  html.div(class: "social")[
    #html.div[
      #html.a(target: "_blank", href: "mailto:mail@liamsnow.com")[
        #html.img(
          src: "/icons/email.svg",
          alt: "Email Icon",
          width: 20,
          height: 20
        )
        Email
      ]
    ]
    #html.div[
      #html.a(target: "_blank", href: "https://www.linkedin.com/in/william-snow-iv-140438169/")[
        #html.img(
          src: "/icons/linkedin.svg",
          alt: "LinkedIn Icon",
          width: 20,
          height: 20
        )
        LinkedIn
      ]
    ]
    #html.div[
      #html.a(target: "_blank", href: "https://github.com/liamsnow")[
        #html.img(
          src: "/icons/github.svg",
          alt: "GitHub Icon",
          width: 20,
          height: 20
        )
        GitHub
      ]
    ]
    #html.div[
      #html.a(target: "_blank", href: "https://github.com/LiamSnow/resume/blob/main/resume.pdf")[
        #html.img(
          src: "/icons/resume.svg",
          alt: "Resume Icon",
          width: 20,
          height: 20
        )
        Resume
      ]
    ]
  ]
}

#let footer() = {
  html.footer[
    #html.div(class: "container")[
      #html.div(class: "left")[
        #social()
      ]
      #html.div(class: "right")[
        © 2025 William Snow IV
        #linebreak()
        #html.div[
          #html.a(target: "_blank", href: "https://github.com/liamsnow/liamsnow.com")[
            #html.img(src: "/icons/code.svg", alt: "Source Code Icon", width: 20, height: 20)
            Source Code
          ]
        ]
      ]
    ]
  ]
}

#let template(
  body,
  styles: (),
  jsonld: none,
) = {
  let title = page.at("title", default: "Liam Snow")
  let desc = page.at("desc", default: "")
  let canonical-url = "https://liamsnow.com" + page.at("url", default: "")
  styles.insert(0, "main");

  html.html(lang: "en")[
    #html.head[
      #html.title[#title]
      #html.meta(charset: "utf-8")
      #html.meta(name: "viewport", content: "width=device-width, initial-scale=1.0")
      #html.meta(name: "description", content: desc)
      #html.meta(name: "author", content: "William Snow IV")
      #html.link(rel: "canonical", href: canonical-url)

      #html.elem("meta", attrs: (property: "og:title", content: title))
      #html.elem("meta", attrs: (property: "og:description", content: desc))
      #html.elem("meta", attrs: (property: "og:type", content: "website"))
      #html.elem("meta", attrs: (property: "og:url", content: canonical-url))
      #html.elem("meta", attrs: (property: "og:site_name", content: "Liam Snow"))

      #html.meta(name: "twitter:card", content: "summary")
      #html.meta(name: "twitter:title", content: title)
      #html.meta(name: "twitter:description", content: desc)

      #html.meta(name: "theme-color", content: "#f0fb29")

      #html.link(rel: "alternate", type: "application/rss+xml", title: "Liam Snow's Blog", href: "/blog/rss.xml")
      #html.link(rel: "alternate", type: "application/rss+xml", title: "Liam Snow's Projects", href: "/projects/rss.xml")

      #if page.at("url", default: "") == "/" {
        html.elem("link", attrs: (rel: "preload", href: "/fonts/DINNextSlabBlack.woff2", ("as"): "font", type: "font/woff2", crossorigin: "anonymous"))
      }

      #html.elem("link", attrs: (rel: "preload", href: "/fonts/SpaceGrotesk-Regular.woff2", ("as"): "font", type: "font/woff2", crossorigin: "anonymous"))
      #html.elem("link", attrs: (rel: "preload", href: "/fonts/SpaceGrotesk-Bold.woff2", ("as"): "font", type: "font/woff2", crossorigin: "anonymous"))

      #for style in styles {
        html.elem("link", attrs: (rel: "preload", href: "/styles/" + style + ".css", ("as"): "style"))
      }

      #html.style()[#read("fonts.css")]

      #for style in styles {
        html.link(rel: "stylesheet", href: "/styles/" + style + ".css")
      }

      #if "css" in sys.inputs {
        html.style()[#sys.inputs.at("css")]
      }

      #html.script(type: "application/ld+json")[#read("schema.json")]

      #if jsonld != none {
        html.script(type: "application/ld+json")[#jsonld]
      }

      #html.script(type: "text/javascript")[#read("preload.js")]
      #html.script(type: "text/javascript")[#read("light_dark.js")]
      #html.script(type: "text/javascript")[#read("header.js")]
      #html.script(type: "text/javascript")[#read("date.js")]
    ]
    #html.body[
      #header()

      #html.main[
        #html.div(id: "content")[
          #body
        ]
      ]

      #footer()
    ]
  ]
}

#let lang-icon(lang) = {
  if lang == "Rust" {
    html.img(src: "/icons/cuddlyferris.svg", alt: "Rust Icon", width: 22, height: 16)
  } else if lang == "SystemVerilog" {
    html.img(src: "/icons/xor.svg", alt: "SystemVerilog Icon", width: 22, height: 15) 
  } else {
    html.img(src: "/icons/code.svg", alt: "Other Programming Language Icon", width: 22, height: 22)
  }
}

#let lang-display(lang) = {
  lang-icon(lang)
  html.p[Language:]
  html.p[#lang]
}

#let post(body) = {
  template(
    [
      #html.div(id: "post-header")[
        #html.div(id: "post-info")[
          = #page.at("title", default: "")
          #page.at("desc", default: "")
        ]

        #html.ul(id: "post-stats")[
          #if "written" in page {
            html.li[
              #html.img(src: "/icons/written.svg", alt: "Written Icon", width: 22, height: 22)
              #html.p[Written:]
              #html.p(class: "date")[
                #page.at("written")
              ]
            ]
          }
        
          #if "updated" in page {
            html.li[
              #html.img(src: "/icons/updated.svg", alt: "Updated Icon", width: 22, height: 22)
              #html.p[Updated:]
              #html.p(class: "date")[
                #page.at("updated")
              ]
            ]
          }

          #if "started" in page {
            html.li[
              #html.img(src: "/icons/rocket_launch.svg", alt: "Started Icon", width: 22, height: 22)
              #html.p[Started:]
              #html.p(class: "date")[
                #page.at("started")
              ]
            ]
          }

          #if "ended" in page {
            html.li[
              #let ended = page.at("ended")
              #if ended == "Now" {
                html.img(src: "/icons/infinite.svg", alt: "Ongoing Project Icon", width: 22)
                html.p[Ongoing]
              } else {
                html.img(src: "/icons/done_all.svg", alt: "Project End Date Icon", height: 22)
                html.p[Ended:]
                html.p(class: "date")[
                  #ended
                ]
              }
            ]
          }

          #if "lang" in page {
            html.li(class: "lang")[
              #lang-display(page.at("lang"))
            ]
          }
        ]

        #html.div(id: "post-quick-links")[
          #for item in page.at("links", default: ()) {
            link-new-tab(item.at(0), item.at(1))
          }
        ]
      ]
      #html.div(id: "post-body")[
        #body
      ]
    ],
    styles: ("post",),
  )
}
