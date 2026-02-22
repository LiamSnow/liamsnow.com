# liamsnow.com

Fast personal website made with Rust & [Typst](https://typst.app/),
hosted on [Helios](https://github.com/oxidecomputer/helios)
[illumos](https://illumos.org/).

## Features
 - All content and layout written in Typst
   - NextJS-esq routing
   - Can query metadata from other pages (see [blog.typ](content/blog.typ))
   - Uses Typst as a library with a custom world for blazingly fast build times
 - Rayon parallel compilation (~10ms dev build time, ~130ms normally)
 - Zero-copy responses via pre-compiled and compressed responses 
 - Hand rolled HTTP/1.1 server
 - Hot reloading / watcher mode for development
 - SCSS support
 - Continuous deployment (GitHub webhooks trigger self-update)
 - Sitemap generation

[See More](https://liamsnow.com/projects/liamsnow_com)

![PageSpeed Insights 100/100 Performance 100/100 Accessibility 100/100 Best Practices 100/100 SEO](https://liamsnow.com/projects/liamsnow_com/pagespeed.png)
![](https://liamsnow.com/projects/liamsnow_com/gt.png)

