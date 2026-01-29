#import "shared/template.typ": template, link
#show: template.with(
  title: "Liam's Blog",
  desc: "Liam Snow's Blog. Programming, systems, backend, Rust and more.",
  styles: ("index",),
  path: "/blog"
)

// #link("RSS", "/blog/rss.xml")

#let posts = toml("blog/routes.toml").routes

#html.div(id: "posts")[
  #posts.map(post => html.a(
    class: "post" + if post.at("highlight", default: false) { " highlight" } else { "" },
    href: "/blog" + post.path
  )[
    #html.h2(class: "title")[#post.title]
    #html.p(class: "desc")[#post.desc]
    #html.p(class: "date")[#post.date]
  ]).join()
]
