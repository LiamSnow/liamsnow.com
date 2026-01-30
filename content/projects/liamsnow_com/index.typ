#import "../../shared/template.typ": post
#show: post.with(
    base: "projects",
    routes: toml("../routes.toml"),
    filename: "liamsnow_com/index.typ"
)

#link("https://github.com/liamsnow/liamsnow.com")[GitHub Repo]

= Why Build This?

I wanted full control over my personal website and to not be limited by existing static site generators (SSGs).
While, I'm sure existing SSGs would have worked fine, making it myself both gave me more freedom and the opportunity to make another Rust project.

= Goals

When starting this project, I set strict performance targets:

- *Static site generation*: Pre-render all content at startup, serve instantly on each request
- *Small page size*: Target minimal transfer size for fast loading and reduced bounce rates
- *High PageSpeed scores*: Optimize for Google's Core Web Vitals, which directly impact SEO rankings
- *Markup-based content*: Write blogs and project pages in markup language
- *Aesthetic but Readable*: It must look good while still being easy to read and navigate

These goals are interconnected. Small page sizes reduce load times, which improves user experience and helps with search engine rankings. Fast initial loads prevent visitors from bouncing before the page even renders. The site currently weighs 13.39kB uncompressed (4.68kB compressed), which helps achieve these objectives.

= Results

#image("pagespeed.png")
#image("gt.png")

The site achieves excellent PageSpeed scores and consistently fast load times. More importantly, the architecture makes adding new content straightforward: write Markdown, push to GitHub, and the site rebuilds automatically via git-ops on my NixOS homelab.

= Development

== Choosing a Templating Engine

I evaluated several Rust templating options before settling on my approach.
I tried
#link("https://crates.io/crates/minijinja")[minijinja] and #link("https://crates.io/crates/handlebars")[handlebars-rust]
first which both are solid libraries. However, I ultimately chose Maud for two reasons: type safety and reducing the number of files in the project.

Maud lets you write HTML directly in Rust, which means the compiler catches errors that would slip through other templating systems. Instead of maintaining separate template files, everything stays in Rust.

Here's an example of a Maud component:

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

I built a template system in `src/template.rs` that takes page metadata and content, wrapping them in complete HTML pages. This approach makes it easy to create reusable components.

== Markup
=== Typst

Initially, I wanted to use Typst for rendering content. Typst is a massive improvement over Markdown, and I use it extensively for personal notes. I spent considerable time trying to make Typst's HTML generation work for this site.

Unfortunately, Typst's HTML support is still early-stage. It lacks equation rendering, syntax highlighting, and other features necessary for my website. There's an #link("https://github.com/typst/typst/issues/721")[active GitHub issue] tracking HTML generation progress. I hope to revisit this once HTML generation matures.

=== Markdown

With Typst not viable, I went with Markdown. I chose #link("https://crates.io/crates/comrak")[comrak]
because it's the same Markdown flavor as GitHub and has extensive plugin support. The syntect plugin handles syntax highlighting, which was essential for my website.

== SCSS

I originally wrote all styling in plain CSS, but nested rules quickly became cumbersome. I set up
#link("https://crates.io/crates/grass")[grass], a Rust SCSS compiler, which simplified the styling workflow.
For development, I created a script in `src/scss.rs` that watches `.scss` files in `static/` and regenerates CSS when files are changed.

Each page has its own SCSS file that imports the main stylesheet. This structure allows me to compile one file per page and inline it easily using `OutputStyle::Compressed`.

== Inlining Everything

One controversial optimization I made was inlining all CSS and JavaScript. This trades away caching benefits for faster first-page loads.

This works because my assets are tiny. The entire site weighs 13.39kB uncompressed (4.68kB compressed). For most visitors, the usage pattern is either landing on the homepage and maybe reading one blog post, or arriving via a direct link to a specific post. In these cases, inlining delivers content faster than making separate requests for tiny CSS and JS files.

The tradeoff is that heavy navigation around the site doesn't benefit from caching. But that's not the typical use case for a personal website, and I optimized for the common path.

For JavaScript, I used #link("https://crates.io/crates/minify-js")[minify-js] to compress the code further before inlining.

== Prefetch on Hover

One feature I'm particularly happy with is prefetching pages when users hover over links. I borrowed this idea from #link("https://www.mcmaster.com/")[McMaster-Carr], which has excellent UX partly because of this technique.

The implementation is straightforward: when you hover over an internal link for more than 100ms, the browser prefetches that page in the background. By the time you click, the page is likely already cached. This makes navigation feel instantaneous.

The 100ms delay prevents prefetching when users accidentally hover over links while scrolling. The script also tracks which URLs have been prefetched to avoid redundant requests. Combined with the small page sizes, this makes the site feel snappy.

== Deployment

The site deploys automatically to my NixOS homelab using git-ops. When I push changes to GitHub, a webhook triggers a rebuild and deployment.
