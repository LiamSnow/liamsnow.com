#metadata((
  title: "Liam's Blog",
  desc: "Liam Snow's Blog. Programming, systems, backend, Rust and more.",
)) <page>

#metadata((blogs: "/blog/")) <query>

#import "_shared/template.typ": template, link, link-new-tab
#show: template.with(styles: ("collection",))

#let posts = {
  sys.inputs.at("blogs", default: ())
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
          #html.img(src: "/icons/written.svg", alt: "Blog start date icon", width: 22, height: 22)
          #html.p[Written:]
          #html.p(class: "date")[
            #post.at("written", default: "")
          ]
        ]
        #html.div[
          #html.img(src: "/icons/updated.svg", alt: "Blog updated icon", width: 22, height: 22)
          #html.p[Updated:]
          #html.p(class: "date")[
            #post.at("updated", default: "")
          ]
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
