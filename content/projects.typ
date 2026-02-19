#metadata((
  title: "Liam's Projects",
  desc: "Liam Snow's Projects. Programming, systems, backend, Rust and more.",
  query: ("/projects/",),
)) <page>

#import "/_shared/template.typ": template, link, link-new-tab, query, lang-display
#show: template.with(styles: ("collection",))

#let posts = {
  query.at(0, default: ())
    .sorted(key: p => p.at("ended", default: "")).rev()
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
          #image("icons/rocket_launch.svg", alt: "Project start date icon")
          #html.p[Started:]
          #html.p(class: "date")[
            #post.at("started", default: "")
          ]
        ]
        #html.div[
          #let ended = post.at("ended", default: "")
          #if ended == "Now" {
            image("icons/infinite.svg", alt: "Ongoing project icon")
            html.p[Ongoing]
          } else {
            image("icons/done_all.svg", alt: "Project end date icon")
            html.p[Ended:]
            html.p(class: "date")[
              #ended
            ]
          }
        ]

        #if "lang" in post {
          html.div(class: "lang")[
            #lang-display(post.at("lang"))
          ]
        }
      ]
    ]
    #html.div(class: "quick-links")[
      #for item in post.at("links", default: ()) {
        link-new-tab(item.at(0), item.at(1))
      }
    ]
  ]).join()
]
