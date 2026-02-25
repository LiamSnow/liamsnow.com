#metadata((
  title: "Wai not WASM?",
  desc: "Experimenting with making Igloo's provider system in WebAssembly",
  written: "2026-02-08",
  updated: "2026-02-08",
  links: (
    ("Project Page", "/projects/igloo"),
    ("Homepage", "https://igloo.rs"),
    ("GitHub", "https://github.com/liamsnow/igloo"),
  ),
  homepage: true
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post

TL;DR WASM is great, it's just not ready for Igloo

= What is WebAssembly (WASM)?
WebAssembly is a "binary instruction format for a stack-based virtual machine."
Basically, think of it like Java bytecode, except many languages
compile to it (Rust, Go, Python, C\#, C, C++, ..).
You get portability, sandboxing, and relative good speeds.

While WebAssembly was originally designed for the web, many people
have started using it outside of that. A few runtimes exist for this purpose:
 - #link-new-tab("wasmtime", "https://docs.wasmtime.dev/"): made by the #link-new-tab("Bytecode Alliance", "https://bytecodealliance.org/"), the same group behind WebAssembly
 - #link-new-tab("wasmer", "https://wasmer.io/"): made by a for-profit company
 - #link-new-tab("wamr", "https://github.com/bytecodealliance/wasm-micro-runtime"): a lightweight runtime also made by the Bytecode Alliance
 - and many more...

The portability and sandboxing that WASM provides makes these appealing
for a lot of use cases. As WASM outside the web starting becoming more
popular, people wanted more (TCP, HTTP, etc).

This is where #link-new-tab("WIT", "https://component-model.bytecodealliance.org/design/wit.html") and #link-new-tab("WASI", "https://wasi.dev/") come in:
 - *WIT* (WebAssembly Interface Type): a language used to define contracts between WASM components. The host can tell the guest which functions it exposes and which functions it wants to call.
 - *WASI* (WebAssembly System Interface): a standard WIT interface for interacting with hardware. It makes the interface for HTTP, then each runtime implements it. This helps keep these interfaces standard and WASM portable.

== Plugins
So, we've outlined two key areas where WASM makes sense: in the web and on servers.

But I think one of the most interesting places for WASM is for plugin systems.
And I'm not the only one who thinks this, #link-new-tab("Helm chose it for theirs", "https://helm.sh/community/hips/hip-0026/").

Why?
 - *Secure by default*: You have to enable and expose things, rather than having them automatically
 - *Polyglot*: Allows many languages by default, without much work
 - *Standard Interface*: State what functions plugins need to have and what they can access in WIT. Then, bindings can be generated into many languages.
 - *Portable*: Extensions are compiled once and run on any platform

= Igloo
Now, from this you'd probably say that Igloo should definitely use WASM.
We might even gain speed, by dropping the IPC. No Unix sockets, just
straight function call -- while being secure.

Well, I thought the same thing. I tried really hard for long time to make
it work, but at some point I had to accept that it's just not ready for this
use case.

We need async, hardware access, good Python support, good Rust support,
and a good developer experience (DX).
I was not able to find a stack that fit all these goals.

== Stack 1: WASM + WIT + WASI
WASI Preview 2 doesn't support async, so I had to start using the unreleased WASIP3.
It's not released, so there's basically no docs for it and requires reading
through code.

The WIT interface is async, but most Rust crates don't understand WASM async yet.
This means Tokio and everything relying on it won't work. This is a huge burn.

WASI exposes a lot, but does not look like it'll support serial ports any time
soon. Using this system means I would implement a WIT interface for serial
ports, and none of the standard libraries in any language would understand that.

In this system, you need to use #link-new-tab("componentize-py", "https://github.com/bytecodealliance/componentize-py")
which will package in CPython. This produces a pretty massive WASM file.
Just like in Rust, a lot of python features don't work.

== Stack 2: WASMER + WAI + WASIX
Wasmer is a for-profit company that wants to move fast.
This is pretty much in direct opposition to the Bytecode Alliance.
So, #link-new-tab("for many reasons", "https://github.com/bytecodealliance/wit-bindgen/issues/306"), Wasmer has their own fork of WIT (WITX, now WAI) and WASI (WASIX).

On the surface, it looks like Wasmer gives us everything we want.
They have many Rust async libraries working and great Python support.
They love making blog posts that have great headlines.

However, the reality is pretty bad. To get async Rust stuff working,
they maintain forks of everything. Actually maintain is not even the
right word, maybe "have forks?" For example, their #link-new-tab("Tokio fork", "https://github.com/wasix-org/tokio") is 1007 commits behind master.

The compile Python to WASM for Wasmer, they tell you to use their
fork of Nuitka, #link-new-tab("py2wasm", "https://github.com/wasmerio/py2wasm").
This is probably the WORST experience I had with any WASM tool ever.
It actually sucks.

Wasmer left me with such a bad taste, I couldn't choose it.

== Stack 3: Extism
Scrambling to find a way to do WASM extensions in Igloo,
I decided to try out #link-new-tab("Extism", "https://extism.org/").

#link-new-tab("Helm developers", "https://helm.sh/community/hips/hip-0026/")
claimed it "was the most mature and well-support Wasm plugin system today."
And I can't help but agree. Extism is an absolute dream to use.
It was by far the best DX.

However, Extism is locked to WASIP1 (no async support).
It is what they say it is -- a great plugin system.
But, this is not what we are making.
Igloo extensions are closer to Linux device drivers than a plugin.

= Conclusion
I tried really, really hard to get WASM to work for Igloo.
I was even willing to hurt DX, limit the capabilities of extensions,
and reduce performance. Even then, it doesn't make sense.

It's just not ready. I need async to be standardized and mature.
I need Rust and Python libraries to support it better.
Until then, Igloo will remain with the process model.
