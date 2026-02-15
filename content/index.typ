#metadata((
  title: "Liam's Website",
  desc: "Liam Snow's personal website! Programming, systems, backend, Rust and more."
)) <page>

#metadata(("projects/", "blog/")) <query>

#import "/_shared/template.typ": template, link, link-new-tab, link-new-tab-highlight, query, social
#show: template.with(
  styles: ("index",),
  jsonld: read("_shared/ld.json"),
)

#html.div(id: "hero")[
  #html.h1[liam#box(html.elem("wbr"))snow]

  #block[
    = About me

    #block[
      Hi, I'm William (Liam) Snow IV
      - CS MS & ECE BS @ Worcester Polytechnic Institute
      - Rust #image("icons/cuddlyferris.svg", width: 18pt), systems, backend, & more
    ]

    *#underline[Nothing] on this website is written by AI* 

    Please give feedback/critiques via #link-new-tab("email", "mailto:mail@liamsnow.com")!

    #social()
  ]
]

#let make-section(name, href, items) = {
  html.section[
    #html.div(class: "header")[
      #link(name, href)
    ]
    #html.ol(class: "posts")[
      #items.map(item => html.li(
        class: if item.at("highlight", default: false) { "highlight" } else { "" },
      )[
        #html.div(class: "info")[
          #link(item.title, item.url)
          #html.p(class: "desc")[#item.at("desc", default: "")]
        ]
        #html.div(class: "quick-links")[
          #for item in item.at("links", default: ()) {
            link-new-tab(item.at(0), item.at(1))
          }
        ]
      ]).join()
    ]
  ]
}

#let projects = {
  query.at(0, default: ()).filter(item => item.at("homepage", default: false))
    .sorted(key: item => item.at("updated", default: "")).rev()
}
#let blogs = {
  query.at(1, default: ()).filter(item => item.at("homepage", default: false))
    .sorted(key: item => item.at("updated", default: "")).rev()
}
#html.div(id: "sections")[
  #make-section("Projects", "projects", projects)
  #make-section("Blog", "blog", blogs)
]
