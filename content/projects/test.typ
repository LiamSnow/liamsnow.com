#import "../shared/template.typ": template
#let metadata = toml("routes.toml").routes.find(p => p.file == "test.typ")
#show: template.with(
  title: metadata.title,
  desc: metadata.desc,
  styles: ("post",),
  path: "/projects/test"
)

= Test Post

Test

== Features

- *Bold text*
- _Italic text_
- `Code snippets`
```rust
fn main() {
    println!("Hello, world!");
}
```
