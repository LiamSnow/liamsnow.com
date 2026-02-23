#metadata((
  title: "Liam's Website",
  desc: "Liam Snow's personal website! Programming, systems, backend, Rust and more.",
)) <page>

#metadata((projects: "/projects/")) <query>
#metadata((blogs: "/blog/")) <query>
#metadata((css: "/styles/index.scss")) <css>

#import "_shared/template.typ": template, link, link-new-tab, link-new-tab-highlight, social, lang-icon
#show: template.with(
  jsonld: read("_shared/ld.json"),
)

#html.div(id: "hero")[
  #html.h1[liam#box(html.elem("wbr"))snow]

  #block[
    = About me

    #block[
      Hi, I'm William (Liam) Snow IV
      - CS MS & ECE BS @ Worcester Polytechnic Institute
      - Rust #html.img(src: "icons/cuddlyferris.svg", width: 23, height: 17, alt: "Rust icon"), systems, backend, & more
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

          #if "lang" in item {
            html.div(class: "lang")[
              #lang-icon(item.at("lang"))
            ]
          }

          #if "ended" in item {
            let ended = item.at("ended")
            if ended == "Now" {
              html.div[#html.img(src: "icons/infinite.svg", alt: "Ongoing project icon", width: 22, height: 22)]
            } else {
              html.div[#html.img(src: "icons/done_all.svg", alt: "Completed project icon", width: 22, height: 22)]
            }
          }
          
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
  sys.inputs.at("projects", default: ()).filter(item => item.at("homepage", default: false))
    .sorted(key: item => item.at("ended", default: "")).rev()
}
#let blogs = {
  sys.inputs.at("blogs", default: ()).filter(item => item.at("homepage", default: false))
    .sorted(key: item => item.at("updated", default: "")).rev()
}
#html.div(id: "sections")[
  #make-section("Projects", "projects", projects)
  #make-section("Blog", "blog", blogs)
]
