#let template(body, title: none) = {
  html.html[
    #html.head[
      #html.meta(charset: "utf-8")
      #html.meta(name: "viewport", content: "width=device-width, initial-scale=1")
      #html.title[#title]
      #html.link(rel: "stylesheet", href: "styles.css")
    ]
    #html.body[
      #body
    ]
  ]
}
