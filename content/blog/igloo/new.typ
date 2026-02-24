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

#import "../../_shared/template.typ": post
#show: post

TL;DR After a lot of battling myself between priorities,
I've finally accepted defeat on making Igloo a Home Assistant competitor.

I would outwardly say I wanted to make Igloo a strong competitor, while
making decisions that contradicted that. I kept making decisions because
they were cool, interesting to code, or fast for the sake of being fast.

I took a big step back, many times, to really think about this.
User's won't notice that Igloo has a better architecture, is faster,
takes less resources, is more secure,
or most of the things that makes Igloo interesting.
I think ultimately Igloo would always be a platform that is a bit
easier to use, a bit more reliable, and has a lot less device support.

But this has already been done!
#link("https://www.openhab.org/")[openHAB] is a much better platform
than Home Assistant. I hear almost only good things about it.
And yet, people pick Home Assistant because it supports their devices
and is just the automatic choice. Even if Igloo was better than
openHAB, I don't think it would become as popular.

= What's Next
So, if not a HASS competitor -- what is Igloo?

It will _mostly_ be my hobby project, used for my home.
I will make decisions simply because I wanted to,
or because I wanted to learn something, or
it was interesting to code, or it was fast.

== What's Leaving
+ *Python support*: Extensions are only Rust
+ *Dashboard System*: No SolidJS, no module federation, no plugins. Just 1 Rust WASM file
+ *Backwards Compatibility*: It will be pure anarchy
+ *Complex Query System*: No matching strings, no glob patterns, nothing. Always by IDs or components
+ *Pi 3 Support*: Dell PowerEdge R430 (my server) will now be the primary target. It _may_ work on ARM, but it's not a goal. The platform is meant to scale on many cores.
+ *Intuitiveness*: There will be no priority towards being intuitive or to be used by non-programmers.
+ *Reliability*: The new program has a lesser focus on reliability. This means we now allow `unsafe`!!
+ *Package Manager*: Packages are manually installed.

== What we Gain
+ *Ridiculous Performance*: No more JSON or Bincode over Unix sockets. We want zero-copy, less context switching, and fully shared device tree (for parallel query execution). 
+ *Faster Dashboard*: No JS, no JSON, no plugin loading.
+ *Better Ergonomics*: Only targeting Rust extensions means no code generation, no designing systems to be cross-language, nothing. Everything is Rust and we leverage that.


