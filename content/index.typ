#metadata((
  title: "Liam's Website",
  desc: "Liam Snow's personal website! Programming, systems, backend, Rust and more."
)) <page>

#metadata(("projects/", "blog/")) <query>

#import "/_shared/template.typ": template, link-new-tab, link-new-tab-highlight, query
#show: template.with(
  styles: ("index",),
  jsonld: read("_shared/ld.json"),
)

#html.div(id: "hero")[
  #html.h1[LIAM#box(html.elem("wbr"))SNOW]

  #block[
    = About me

    Hi, I'm William (Liam) Snow IV
    - CS MS & ECE BS @ Worcester Polytechnic Institute
    - Rust 🦀, systems, & backend

    *#underline[Nothing] on this website is written by AI* 

    #link-new-tab("EMAIL", "mailto:mail@liamsnow.com")
    #link-new-tab("LINKEDIN", "https://www.linkedin.com/in/william-snow-iv-140438169/")
    #link-new-tab("GITHUB", "https://github.com/liamsnow")
    #link-new-tab-highlight("RESUME", "https://github.com/LiamSnow/resume/blob/main/resume.pdf")
  ]
]

#let make-section(name, items) = {
  html.section[
    #html.div(class: "header")[
      #html.span[#name]
      #html.span[■]
    ]
    #html.div(class: "posts")[
      #items.map(item => html.a(
        class: if item.at("highlight", default: false) { "highlight" } else { "" },
        href: item.url
      )[
        == #item.title
        #html.p(class: "desc")[#item.at("desc", default: "")]
        #html.p(class: "date")[#item.at("date", default: "")]
      ]).join()
    ]
  ]
}

#let projects = query.at(0, default: ()).filter(item => item.at("homepage", default: false)).sorted(key: item => item.at("date", default: "")).rev()
#let blogs = query.at(1, default: ()).filter(item => item.at("homepage", default: false)).sorted(key: item => item.at("date", default: "")).rev()
#html.div(id: "sections")[
  #make-section("PROJECTS", projects)
  #make-section("BLOG", blogs)
]
