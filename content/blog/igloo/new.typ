#metadata((
  title: "An update on Igloo",
  desc: "Changing priorities and more to come!",
  written: "2026-02-24",
  updated: "2026-02-24",
  links: (
    ("Project Page", "/projects/igloo"),
    ("Homepage", "https://igloo.rs"),
    ("GitHub", "https://github.com/liamsnow/igloo"),
  ),
  homepage: true
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post

*TL;DR* After a lot of thinking, I've decided to stop working on Igloo V2. Instead, I'm moving to a V3, which is not intended to be a direct HASS competitor and instead only focused on niche group power users who love Rust.

From the beginning, I have valued rigor and learning above all else in Igloo.
I would always be happier working another year on the project to make something better.
This is why I spent time
#link-new-tab("exploring WASM", "./wasm") and
#link-new-tab("other things", "./shmem").
But at some point, I realized all these explorations came from a place of fundamentally not being happy with Igloo V2:
 - I never wanted to support Python and disagree with writing device drivers in Python
 - I hated that providers (device drivers) were separate processes communicating over Unix Sockets, instead of built-in, like on Linux
 - Focusing on #link-new-tab("Penguin", "./penguin") for automations made sense for the audience, but I would always just prefer Rust
 - I wanted Igloo to help me easily build a complex smart home, but focusing on making an intuitive platform pushed that goal aside
 - I want to hold Igloo's code to a high standard and find the best architecture possible. Although, most of the audience probably doesn't care about this and if anything, they'd dislike that it's so different from HASS because making porting hard

I had a long internal debate about the future of the project.
If I truly wasn't happy with the project, maybe I just needed to go back to the drawing board.
#link-new-tab("Bryan Cantrill's \"Systems Software in the Large\" blog post", "https://oxide.computer/blog/systems-software-in-the-large") really clicked with me at this moment. Igloo feels like a project that will be "never completely solved, but also not even really understood until implementation is well underway."
And this explains why Igloo's already on a V2, and why a V3 would make sense. I couldn't know the problems I was going to encounter, until I started implementing.

I spent a long time outlining what I truly wanted from the project, and through this, I think I found the right place for it.

= The New Version (V3)
Igloo V3 is a DIY smart home Rust library. It provides a cohesive system and collection of crates to help code your own smart home program. Its goals are:
 + *Cohesion*: provides cohesion between device drivers to allow for an ecosystem to exist around Igloo. IE a dashboard crate for Igloo should automatically work with and any and new device drivers
 + *Power*: doesn't limit what you can do in your smart home, and makes advanced functionality easy
 + *Ergonomics*: is a good experience to use

This is a drastic shift, but it's ultimately what I want to make and use most.
The audience is a lot smaller -- a niche of people who both care a lot about smart homes and know Rust. But that's okay.
