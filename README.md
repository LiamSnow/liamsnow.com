# liamsnow.com

![](https://liamsnow.com/static/images/liamsnow_com_pagespeed.png)
![](https://liamsnow.com/static/images/liamsnow_com_gt.png)

Fast personal website made in Rust with Axum.

 - 100/100 performance and 100/100 accessibility on [PageSpeed Insights](https://pagespeed.web.dev)
 - 300ms speed index
 - 7-10kB pages (except longer posts)

## Features
 - Pre-generates all pages
 - Blog posts and projects generated from markdown files using [comrak](https://crates.io/crates/comrak) and [syntect](https://crates.io/crates/syntect) syntax highlighting
 - Minifies and inlines CSS & JS
 - Sitemap and RSS feed generation
 - Sass compiler (using [grass](https://crates.io/crates/grass)) + file watching feature with `-F dev`
 - Clean HTML using [maud](https://crates.io/crates/maud) macros
