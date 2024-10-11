# liamsnow.com

Making vim in the browser using Rust (WebAssembly)!!

## Libraries
Leptos + Axum

## Display Protocol
TODO

## Vim
 - Viewing only (IE only Normal and Visual/Visual Block modes).
 - Simulates my Neovim setup (ie has some extra plugins like sneak)
 - Must support link navigation
 - Must support clipboard

## Data
Create a simple data structure that is shared
between HTML and Vim versions.

There will be 2 types of pages: menus and files.
A menu is basically start up page [like this](https://github.com/nvimdev/dashboard-nvim).

### Current Plan
`liamsnow.com`: menu
 - About (file)
 - Projects (menu)
 - Blog (menu)

## Fallback
 - Has HTML fallback for for users at html.liamsnow.com (linked from liamsnow.com)
 - For Search Engines:
     - HTML page is served to robots at liamsnow.com (based on user agent)
     - html.liamsnow.com blocks all robots

