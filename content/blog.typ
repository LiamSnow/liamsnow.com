#metadata((
  title: "Liam's Blog",
  desc: "Liam Snow's Blog. Programming, systems, backend, Rust and more."
)) <page>

#metadata("blog/") <query>

#import "/_shared/template.typ": template, link, link-new-tab, query
#show: template.with(styles: ("collection",))

#let posts = {
  query.at(0, default: ())
    .sorted(key: p => p.at("updated", default: "")).rev()
}

#html.ol(class: "posts")[
  #posts.map(post => html.li(
    class: if post.at("highlight", default: false) { " highlight" } else { "" }
  )[
    #html.div(class: "top")[
      #html.div(class: "info")[
        #link(post.title, post.url)
        #post.at("desc", default: "")
      ]
      #html.div(class: "stats")[
        #html.div[
          #image("icons/written.svg")
          #html.p[Written:]
          #html.p[#post.at("written", default: "")]
        ]
        #html.div[
          #image("icons/updated.svg")
          #html.p[Updated:]
          #html.p[#post.at("updated", default: "")]
        ]
      ]
    ]
    #html.div(class: "quick-links")[
      #for item in post.at("links", default: ()) {
        link-new-tab(item.at(0), item.at(1))
      }
    ]
  ]).join()
]
