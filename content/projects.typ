#import "shared/template.typ": template, link
#show: template.with(
  title: "Liam's Projects",
  desc: "Liam Snow's Projects. Programming, systems, backend, Rust and more.",
  styles: ("index",),
  path: "/projects"
)

// #link("RSS", "/projects/rss.xml")

#let posts = toml("projects/routes.toml").routes

#html.div(id: "posts")[
  #posts.map(post => html.a(
    class: "post" + if post.at("highlight", default: false) { " highlight" } else { "" },
    href: "/projects" + post.path
  )[
    #html.h2(class: "title")[#post.title]
    #html.p(class: "desc")[#post.desc]
    #html.p(class: "date")[#post.date]
  ]).join()
]
