#metadata((
  title: "igloo",
  desc: "A secure, fast, & intuitive smart home platform",
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

#import "../_shared/template.typ": post 
#show: post

= Background

#link("https://www.home-assistant.io/")[Home Assistant] is a smart home platform that can
connect nearly any smart home product. It breaks down vendor lock-in and allows you
to have one singular platform to manage your entire home. On top of this, it allows
for scripting, custom dashboards, and automations. It makes smart homes fun and powerful.

Home Assistant is the reason I got interested in smart homes. I think it's an amazing
tool, but at the same time it has a lot of flaws. I think its overcomplicated, has a bad architecture, and is unreliable.

My original goal for Igloo was to make a competitor.
However, competiting with HASS's thousands of extensions is probably an
impossible feat. If #link("https://www.openhab.org/")[openHAB] couldn't succeed,
I don't think I could. Users want a platform that supports all their devices,
not one that has a better underlying architecture or type-system.

So, Igloo is and will remain my hobby project.
My goal here is to make the best architecture and fastest platform.
It will be purely Rust, all the way through -- core, extensions, & dashboard.

= Goals
 + *Low Latency & High Throughput*: You might say "All the latency is IO-bound" and "My device can't respond to 10 million requests/sec." Doesn't matter -- Igloo will handle it.
 + *Robust & Reliable*: The core should never crash. It should be able to handle extension crashes and restarting them gracefully.
 + *Secure*: Extensions should have fine-grained permissions. We should be able to block WAN and/or LAN, file system access, and anything else malicious.

= Updates/Blog

#let posts = {
  sys.inputs.at("blogs", default: ())
    .sorted(key: p => p.at("updated", default: "")).rev()
}

#html.div(class: "posts")[
  #posts.map(post => html.a(href: post.url, class: "post")[
    #html.div(class: "title")[#post.title]
    #html.div(class: "desc")[#post.at("desc", default: "")]
  ]).join()
]
