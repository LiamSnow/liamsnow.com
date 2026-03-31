#metadata((
  title: "igloo",
  desc: "A powerful DIY smart home Rust library",
  started: "2025-01-01",
  ended: "Now",
  lang: "Rust",
  links: (
    ("Homepage", "https://igloo.rs"),
    ("GitHub", "https://github.com/liamsnow/igloo"),
  ),
  homepage: true,
)) <page>

#metadata((blogs: "/blog/igloo/")) <query>

#import "../_shared/template.typ": post, link-new-tab 
#show: post

= Background

#link-new-tab("Home Assistant", "https://www.home-assistant.io/") (HASS)
is a smart home platform that can connect nearly any smart home product. It breaks down vendor lock-in and gives you a single platform to manage your entire home. On top of this, it supports scripting, custom dashboards, and automation. It makes smart homes fun and powerful.

Home Assistant got me interested in smart homes, and while I think it’s an amazing tool, it has many flaws. In summary:
 - *Resource Hog*: Takes a lot of system resources. In my case, 0.7-1GB of RAM, which is ridiculous for what it's doing. While they claim it can run fine on a RPi 3B+, my experience and the experience of others differs (#link-new-tab("1", "https://community.home-assistant.io/t/twenty-two-things-i-wish-id-known/585631")).
 - *Unintuitive*: HASS has a ton of features, a poorly laid out UI, and essentially turned YAML into a programming language. (#link-new-tab("1", "https://www.thesmarthomehookup.com/home-assistant-beginners-guide-2020-installation-addons-integrations-scripts-scenes-and-automations/"), #link-new-tab("2", "https://community.home-assistant.io/t/twenty-two-things-i-wish-id-known/585631"))
 - *Security*: Has had many security vulnerabilities, and consistently has bad responses (#link-new-tab("1", "https://www.elttam.com/blog/pwnassistant/"))
 - *Architecture*: This is much more of personal gripe than anything. I disagree with implementing device drivers & protocols in Python. Furthermore, it uses JSON strings all over the place which is not only slow, but unreliable. The history system also uses JSON strings and puts them in SQL database, making it take a lot more space than it needs to.

While the Home Assistant developers and community are working hard to improve it, I think the real solution is a complete rewrite and re-thinking of how it works. This is why I am building Igloo.

= Goals
Initially, Igloo was meant to be a direct HASS replacement (V2), appealing to a wide audience. This is no longer the goal of it (V3), and is instead made for a niche audience of power users who know Rust.

Igloo is a DIY smart home Rust library. It provides a cohesive system and collection of crates to help code your own smart home program. Its goals are:
 + *Cohesion*: provides cohesion between device drivers to allow for an ecosystem to exist around Igloo. IE a dashboard crate for Igloo should automatically work with and any and new device drivers
 + *Power*: doesn't limit what you can do in your smart home, and makes advanced functionality easy
 + *Ergonomics*: is a good experience to use

#let cutoff = "2026-02-24"

#let all-posts = {
  sys.inputs.at("blogs", default: ())
    .sorted(key: p => p.at("written", default: ""))
}

#let new-posts = all-posts.filter(p => p.at("written", default: "") >= cutoff).rev()
#let old-posts = all-posts.filter(p => p.at("written", default: "") < cutoff).rev()

= V3 Updates

#html.div(class: "posts")[
  #new-posts.map(post => html.a(href: post.url, class: "post")[
    #html.div(class: "title")[#post.title]
    #html.div(class: "desc")[#post.at("desc", default: "")]
  ]).join()
]


= History
As I mentioned earlier, Igloo V2 was initially intended to be a direct replacement for HASS. These posts are all related to that version:

#linebreak()

#html.div(class: "posts")[
  #old-posts.map(post => html.a(href: post.url, class: "post")[
    #html.div(class: "title")[#post.title]
    #html.div(class: "desc")[#post.at("desc", default: "")]
  ]).join()
]
