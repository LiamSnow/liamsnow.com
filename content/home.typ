#import "shared/template.typ": template

#show: template.with(title: "Liam Snow")

= Liam Snow

TODO

== Recent Posts

#let posts = toml("blog/routes.toml").routes
#for post in posts.slice(0, calc.min(3, posts.len())) [
  === #link("/blog" + post.path)[#post.title]
  #emph(post.date) - #post.desc
]

#link("/blog")[View all posts →]



