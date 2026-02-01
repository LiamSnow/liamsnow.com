#metadata((
  title: "Liam's Blog",
  desc: "Liam Snow's Blog. Programming, systems, backend, Rust and more."
)) <page>

#metadata("blog/") <query>

#import "/_shared/template.typ": template, link, query
#show: template.with(styles: ("collection",))

#let posts = query.at(0, default: ()).sorted(key: p => p.at("date", default: "")).rev()

#html.div(class: "posts")[
  #posts.map(post => html.a(
    class: if post.at("highlight", default: false) { " highlight" } else { "" },
    href: post.url
  )[
    #html.h3[#post.title]
    #html.p(class: "desc")[#post.at("desc", default: "")]
    #html.p(class: "date")[#post.at("date", default: "")]
  ]).join()
]
