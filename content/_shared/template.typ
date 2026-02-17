#let path = sys.inputs.at("path", default: "/")
#let page = json.decode(sys.inputs.at("page", default: "{}"))
#let query = json.decode(sys.inputs.at("query", default: "[]"))

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
            #image("../icons/moon.svg", alt: "Enable dark mode icon")
          ]
          #html.span(class: "sun", style: "display: none")[
            #image("../icons/sun.svg", alt: "Enable light mode icon")
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
        #image("../icons/email.svg", alt: "Email Icon")
        Email
      ]
    ]
    #html.div[
      #html.a(target: "_blank", href: "https://www.linkedin.com/in/william-snow-iv-140438169/")[
        #image("../icons/linkedin.svg", alt: "LinkedIn Icon")
        LinkedIn
      ]
    ]
    #html.div[
      #html.a(target: "_blank", href: "https://github.com/liamsnow")[
        #image("../icons/github.svg", alt: "GitHub Icon")
        GitHub
      ]
    ]
    #html.div[
      #html.a(target: "_blank", href: "https://github.com/LiamSnow/resume/blob/main/resume.pdf")[
        #image("../icons/resume.svg", alt: "Resume Icon")
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
            #image("../icons/code.svg", alt: "Source Code Icon")
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
  let canonical-url = "https://liamsnow.com" + path

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

      #html.elem("link", attrs: (rel: "preload", href: "/fonts/DINNextSlabBlack.woff2", ("as"): "font", type: "font/woff2", crossorigin: "anonymous"))
      #html.elem("link", attrs: (rel: "preload", href: "/fonts/SpaceGrotesk-Regular.woff2", ("as"): "font", type: "font/woff2", crossorigin: "anonymous"))
      #html.elem("link", attrs: (rel: "preload", href: "/fonts/SpaceGrotesk-Bold.woff2", ("as"): "font", type: "font/woff2", crossorigin: "anonymous"))

      #for style in styles {
        html.elem("link", attrs: (rel: "preload", href: "/styles/" + style + ".css", ("as"): "style"))
      }

      #for style in styles {
        html.link(rel: "stylesheet", href: "/styles/" + style + ".css")
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
    image("/icons/cuddlyferris.svg", alt: "Rust Icon")
  } else if lang == "SystemVerilog" {
    image("/icons/xor.svg", alt: "SystemVerilog Icon") 
  } else {
    image("/icons/code.svg", alt: "Other Programming Language Icon")
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
              #image("/icons/written.svg", alt: "Written Icon")
              #html.p[Written:]
              #html.p(class: "date")[
                #page.at("written")
              ]
            ]
          }
        
          #if "updated" in page {
            html.li[
              #image("/icons/updated.svg", alt: "Updated Icon")
              #html.p[Updated:]
              #html.p(class: "date")[
                #page.at("updated")
              ]
            ]
          }

          #if "started" in page {
            html.li[
              #image("/icons/rocket_launch.svg", alt: "Started Icon")
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
                image("/icons/infinite.svg", alt: "Ongoing Project Icon")
                html.p[Ongoing]
              } else {
                image("/icons/done_all.svg", alt: "Project End Date Icon")
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
