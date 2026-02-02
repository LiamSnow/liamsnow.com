#metadata((
  title: "liamsnow.com",
  desc: "Fast personal website made with Rust & Typst, hosted on Helios illumos.",
  date: "2026-01-31",
  homepage: true
)) <page>

#import "/_shared/template.typ": post
#show: post

#link("https://github.com/liamsnow/liamsnow.com")[GitHub Repo]

liamsnow.com is fast website written in Rust
that has all content and layout written in #link("https://typst.app")[Typst].
There is no HTML, only Typst and SCSS.

= Why Build This?

While many SSGs would work:
 + I am very particular with what I want
 + I want bleeding edge performance
 + It needs to run on illumos
 + I want to write everything in Typst

= Why Typst?
For about a year in college I wrote all my notes in Obsidian.
After so much frustration with Obsidian being slow,
having to pay for syncing to mobile, and it being exceptionally hard
to write extensions (at the time), I decided I needed something else.

I started just writing my notes in Markdown, using
#link("https://pandoc.org/")[pandoc] to compile it.
I made a Neovim extension to automatically open up a browser preview of this.
I experimented with writing all my notes in LaTeX, but it was just too
tedious and time consuming.

Eventually I discovered Typst. It was the best of both worlds,
the power of LaTeX (and more) with the ease of Markdown.
It completely transformed my note taking experience.

When I was making the #link("liamsnow_com/v1")[original version]
of my website, Typst HTML export was just not ready, so
I set it up to use Markdown.

I used this for about half a year, but, I just always had this
feeling that I had to do this in Typst.
It would give me so much more power and make writing easier.
Typst HTML export was in better state, but still not ready.
It doesn't have multi-page export and
#link("https://github.com/typst/typst/pull/7783")[randomly breaks].
However, I persevered through and was able to get it all working.

= Indexer
== Discovery
All content in written in a `content/` folder.
At the start of the program, it will walk through
content and index it. Mapping:
 - `index.typ` -> `liamsnow.com/`,
 - `projects/igloo.typ` -> `liamsnow.com/projects/igloo`
 - `styles/home.scss` -> `liamsnow.com/styles/home.css` (will be compiled)
 - etc.
All other files keep their extensions. Files prefixed with `_` are ignored.

== Metadata
Each `.typ` file will have metadata tag at the top of the file
to describe the title and other useful information:

```typst
#metadata((title: "..", ..)) <page>
```

After discovery, we spawn up a bunch of tokio tasks
to grab this metadata from each page in parallel
(using `typst query ..`).

== Queries
Some pages, like `/`, `/blog`, etc. need to get information
about other pages. Ex. `/blog` needs a list of all blogs.

I have introduced a simple query system, where `.typ` can have
the following at the top of their file:

```typst
#metadata("blog/") <query>
```

Which will then return a response (via `sys.inputs`)
of the metadata of all pages in `blog/`. 
This metadata also includes the mapped url from discovery
so it can link to those pages.

= Compiler
After indexing, we spawn up a bunch of tokio tasks to process
each file in parallel. This will do a few things:
 + Compile `.typ` files
 + Compile `.scss`, `.sass`, `.css` files using #link("https://crates.io/crates/grass")[grass] (which also compacts them)
 + Pre-compress everything using #link("https://en.wikipedia.org/wiki/Brotli")[Brotli]

= Hot Reloading
Having a hot reload system really helps with writing posts.
So I set up a pretty simple system for this:
 + Inject some code into the website which will connect to a websocket. When it recieves a message, refresh the page.
 + Watch for any file modifications in `content/` (create, remove, modify)
   + Upon change, rebuild, then notify websockets

I couldn't use `typst watch` for a few reasons:
 + Typst doesn't know about our dependencies from our custom query system
 + We need to compile things besides just Typst files

= Continuous Deployment
It would be really annoying if I had to SSH into my server, login
to the zone, git pull, recompile, and restart the service for every
change to my website. So, I created a pretty simple CD system.

I set up a GitHub webhook which POSTs to `liamsnow.com/_update`.
Upon receiving this it will:
 + Verify the sig/secret
 + Git pull
 + Recompile
 + Stop the service (#link("https://en.wikipedia.org/wiki/Service_Management_Facility")[SMF] will restart it)


= Prefetch on Hover
I've always loved how fast and snappy #link("https://www.mcmaster.com/")[McMaster-Carr]
is. So I decided to bring over one of their best features - prefetching pages on hover.
It prefetches pages when you hover over them, so that when you actually
do navigate there, its already cached.

= Results

I'm super happy with the results. Its fast but also
has a great experience to use. I get to write posts
in Typst, with hot reloading, and continuous deployment.

#html.img(src: "liamsnow_com/pagespeed.png")
#html.img(src: "liamsnow_com/gt.png")

