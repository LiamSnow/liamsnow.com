---
title: liamsnow.com
desc: Fast personal website made in Rust with Axum
date: 2025-07-04
homepage: true
---

[GitHub Repo](https://github.com/liamsnow/liamsnow.com)

# Goals
 - Basically fully SSR
 - Small page size
 - Highly ranked on [PageSpeed Insights](https://pagespeed.web.dev/)
 - Blogs and project pages written in Markdown or Typst

These are pretty strict goals, but I think they really led me in the
right direction. After trying out a few ways to do this with different
templating engines, I eventually landed on Axum + Maud which I think
is a great combination. It allows me to keep everything in Rust,
have type-safe templates, and just be really fast.


# Results

![](/static/images/liamsnow_com_pagespeed.png)
![](/static/images/liamsnow_com_gt.png)

# Development

## Typst
I spent a while trying to get Typst HTML generation to work, but its really
just not there yet. I was firm on having features like code
syntax highlighting and displaying math well which Typst just can't achieve
its current state.

Its kind of sad, I think Typst is an amazing tool and love using it for my
notes. I hope that soon I will be able to transition over.

## Markdown
Since Typst wasn't going to work, the obvious choice was to go with Markdown.
I chose [comrak](https://crates.io/crates/comrak) since it matches GitHub
and has a ton of plugins. Notably, the [syntect](https://crates.io/crates/syntect)
plugin which makes it really easy to do syntax highlighting.

## Maud
As I said before I think Maud is just an amazing experience. It is really easy
to work with and I was able to make some really cool functions. I have
`src/template.rs` which takes in some page metadata, content, etc. and wraps
it in an HTML page. Then I also made some components like header:

```rust
fn header(path: &str) -> Markup {
    html! {
        header {
            .container {
                .left {
                    (link("IV", &get_base_url(path)))
                }
                .nav.desktop {
                    (link("BLOG", "/blog"))
                    (link("PROJECTS", "/projects"))
                }
                .nav.mobile {
                    button { "MENU" }
                }
            }
        }
    }
}
```

## SCSS
Originally I had written everything with CSS, but with all the nested rules it
was becoming really annoying. I setup [grass](https://crates.io/crates/grass)
which makes it super easy. Then I made a simple script in `src/scss.rs` which
watches the `.scss` files in `static/` and generates the files on change. 

## Optimizations
Since I was looking for fast load times and I knew that my CSS and JS were
very small, I decided to inline everything. While this does loose on caching,
it enables excelling first-page load speed, which I think is more important
for most visitors.

Each page has its own
SCSS file which imports the main SCSS file. This way I can just have `grass`
process one file and easily inline it with `OutputStyle::Compressed`.
Its pretty similar for JS, except I used `minify-js` which does a bit more
optimization. 

Then I added a few things like preload links and a simple Javascript file to prefetch
pages when hovering over them (like [McMaster-Carr](https://www.mcmaster.com/)). 
