#let posts = toml("blog/routes.toml").routes

= Blog

#for post in posts [
  == #link("/blog" + post.path)[#post.title]
  #emph(post.date) - #post.desc
]
