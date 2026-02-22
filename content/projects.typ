#metadata((
  title: "Liam's Projects",
  desc: "Liam Snow's Projects. Programming, systems, backend, Rust and more.",
)) <page>

#metadata((projects: "/projects/")) <query>

#import "_shared/template.typ": template, link, link-new-tab, lang-display
#show: template.with(styles: ("collection",))

#let posts = {
  sys.inputs.at("projects", default: ())
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
          #html.img(src: "/icons/rocket_launch.svg", alt: "Project start date icon")
          #html.p[Started:]
          #html.p(class: "date")[
            #post.at("started", default: "")
          ]
        ]
        #html.div[
          #let ended = post.at("ended", default: "")
          #if ended == "Now" {
            html.img(src: "/icons/infinite.svg", alt: "Ongoing project icon", width: 22, height: 22)
            html.p[Ongoing]
          } else {
            html.img(src: "/icons/done_all.svg", alt: "Project end date icon", width: 22, height: 22)
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
