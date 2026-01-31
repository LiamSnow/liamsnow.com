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
        #link("BLOG", "/blog")
        #link("PROJECTS", "/projects")
      ]
    ]
  ]
}

#let footer() = {
  html.footer[
    #html.div(class: "container")[
      #html.div(class: "left")[
        #link-new-tab("EMAIL", "mailto:mail@liamsnow.com")
        #link-new-tab("LINKEDIN", "https://www.linkedin.com/in/william-snow-iv-140438169/")
        #link-new-tab("GITHUB", "https://github.com/liamsnow")
        #link-new-tab-highlight("RESUME", "https://github.com/LiamSnow/resume/blob/main/resume.pdf")
      ]
      #html.p(class: "right")[
        © 2025 William Snow IV
        #linebreak()
        #link-new-tab("Made with Rust 🦀", "https://github.com/liamsnow/liamsnow.com")
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

      #html.elem("link", attrs: (rel: "preload", href: "/fonts/SpaceMono-Regular.woff2", ("as"): "font", type: "font/ttf", crossorigin: "anonymous"))
      #html.elem("link", attrs: (rel: "preload", href: "/fonts/SpaceMono-Bold.woff2", ("as"): "font", type: "font/ttf", crossorigin: "anonymous"))
      #html.elem("link", attrs: (rel: "preload", href: "/fonts/SpaceGrotesk-Regular.woff2", ("as"): "font", type: "font/otf", crossorigin: "anonymous"))

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

#let post(body) = {
  // derive base from path (ex., /blog/igloo/ecs → blog)
  let parts = path.split("/").filter(p => p != "")
  let base = if parts.len() > 0 { parts.at(0) } else { "" }

  let title = page.at("title", default: "")
  let date = page.at("date", default: "")

  let post-header = [
    #html.a(class: "post-back", href: "/" + base)[← #base]
    #html.div(class: "post-title")[#title]
    #html.p(class: "post-date")[#date]
  ]

  template(
    [
      #post-header
      #body
    ],
    styles: ("post",),
  )
}
