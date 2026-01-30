# liamsnow.com

Expressive & fast personal website made in Rust with Typst.

![PageSpeed Insights 100/100 Performance 100/100 Accessibility 100/100 Best Practices 100/100 SEO](https://liamsnow.com/static/images/liamsnow_com_pagespeed.png)
![](https://liamsnow.com/static/images/liamsnow_com_gt.png)

## Features
 - All content and layout written in Typst
   - metadata queries integrations (see [home.typ](content/home.typ))
 - Watcher mode for development
 - Parallel compilation (~250ms startup time)
 - Pre-compressed responses
 - Directly uses [hyper](https://hyper.rs/) :)
 - SCSS & SASS support
 - Sitemap generation

[See More](https://liamsnow.com/projects/liamsnow_com)

## TODO
 1. Remove `routes.toml` and use NextJS like routing
 2. Page metadata (title, etc) placed in metadata
 3. Place article metadata in separate metadata
 4. Allow pages (in metadata) to request to query other pages (ex. blog.typ needs to grab all blogs)
 5. Allow nested blogs and projects
 6. Make zettelkasten like system
    - we want projects/igloo to have cards for each blog maybe?


