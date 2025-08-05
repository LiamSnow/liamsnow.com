---
title: liamsnow.com
desc: Fast personal website made in Rust with Axum
date: 2025-07-04
homepage: true
---

[GitHub Repo](https://github.com/liamsnow/liamsnow.com)

I spent forever, an I mean a few years not making a personal website because
I could never decide on what I wanted it to look like.

Recently I decided that I just needed to make something and stick with it.
I had a prototype I was working on last year which is actually in the same
repository.

When I started making my new website I tried out a few different approaches,
but eventually realized I think I had it right from the start. The combination
of Axum and Maud is really nice for simple websites, especially if you looking
for speed and clean code.

## Goals
 - Basically fully SSR
 - Small page size
 - Highly ranked on [PageSpeed Insights](https://pagespeed.web.dev/)
 - Blogs and project pages written in Markdown or Typst

## Development

### Typst
I spent awhile trying to get Typst HTML generation to work, but its really
just not there yet. I was firm on having features like code
syntax highlighting and displaying math well which Typst just can't achieve
its current state.

Its kind of sad, I think Typst is an amazing tool and love using it for my
notes. I hope that soon I will be able to transition over.

### Markdown
Since Typst wasn't going to work, the obvious choice was to go with Markdown.
I chose [comrak](https://crates.io/crates/comrak) since it matches GitHub
and has a ton of plugins. One of them I was really excited about was
[syntect](https://crates.io/crates/syntect) which makes it really easy
to do syntax highlighting.

It was pretty easy to get setup and running. I some code in `src/post.rs`
which will read a directory (either `projects/` or `blog/`), parse all
the markdown files, and generate the page content.

### Maud
As I said before I think Maud is just an amazing experience. Its really easy
to work with and I was able to make some really cooler functions. I have
`src/template.rs` which takes in some page metadata, content, etc. and wraps
it in a HTML page. Then I also made some components like header:

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

### SCSS
Originally I had written everything with CSS, but with all the nested rules it
was becoming really annoying. I setup [grass](https://crates.io/crates/grass)
which makes it super easy. Then I made a simple script in `src/scss.rs` which
watches the `.scss` files in `static/` and generates the files on change. 

### Optimizations
Since I was looking for fast load times and I knew that my CSS and JS were
very small, I decided to inline everything. Each page has its own
SCSS file which imports the main SCSS file. This way I can just have `grass`
process one file and easily inline it with `OutputStyle::Compressed`.
Its pretty similar for JS, except I used `minify-js` which does a bit more
optimization. 

Then I added a few things like preload links and a simple Javascript file to prefetch
pages when hovering over them (like McMaster). 

Otherwise, I didn't actually have to do much more optimization. The server
already has the pages generated a boot time, so its just shipping off a string
as fast as it can.

