#metadata((
  title: "Liam's Notes",
)) <page>

#metadata((notes: "/notes/")) <query>

#import "_shared/template.typ": template, link, link-new-tab
#show: template.with(styles: ("collection",))

#html.div(class: "preface")[
  = Liam's Notes

  I spend a lot of time reading and listening about programming, but had never taken notes. I've been making an effort to do this, and hopefully this collection will grow into a great database of knowledge.
]

#let posts = {
  sys.inputs.at("notes", default: ())
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

      #html.div(class: "quick-links")[
        #for item in post.at("links", default: ()) {
          if "." in item.at(1) {
            link-new-tab(item.at(0), item.at(1))
          } else {
            link(item.at(0), item.at(1))
          }
        }
      ]
    ]
  ]).join()
]

