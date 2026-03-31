#metadata((
  title: "TODO",
  desc: "Things I've read or listened to (or want to) and plan to take notes on.",
  homepage: false,
)) <page>

#import "../_shared/template.typ": post, link-new-tab 
#show: post

== Graydon Hoare
+ #link-new-tab("The Rust I Wanted Had No Future", "https://graydon2.dreamwidth.org/307291.html")
+ #link-new-tab("Abstract machines and the compilers that love/hate them", "https://graydon2.dreamwidth.org/264181.html")
+ #link-new-tab("Rust is mostly safety", "https://graydon2.dreamwidth.org/247406.html")
+ TODO add more -- lots of interesting stuff here

== kernel.org
+ #link-new-tab("What is RCU? -- \"Read, Copy, Update\"", "https://docs.kernel.org/RCU/whatisRCU.html")
+ #link-new-tab("Sequence counters and sequential locks", "https://docs.kernel.org/locking/seqlock.html")
+ TODO also add more here

== Cliffle (Cliff L. Biffle)
+ #link-new-tab("Learn Rust the Dangerous Way", "https://cliffle.com/p/dangerust/")
+ #link-new-tab("How to think about `async`/`await` in Rust", "https://cliffle.com/blog/async-inversion/") (combine into bigger note about Rust futures & async)
+ #link-new-tab("Mutex without lock, Queue without push: cancel safety in lilos", "https://cliffle.com/blog/lilos-cancel-safety/")
+ #link-new-tab("Composing concurrency in drivers", "https://cliffle.com/blog/composed-concurrency-in-drivers/")
+ #link-new-tab("On Hubris And Humility", "https://cliffle.com/blog/on-hubris-and-humility/")
+ #link-new-tab("Why Rust mutexes look like they do", "https://cliffle.com/blog/rust-mutexes/")
+ #link-new-tab("The First-Mover Allocator Pattern", "https://cliffle.com/blog/rust-first-mover/")
+ #link-new-tab("Making really tiny WebAssembly graphics demos", "https://cliffle.com/blog/bare-metal-wasm/")
+ #link-new-tab("The Typestate Pattern in Rust", "https://cliffle.com/blog/rust-typestate/")
+ #link-new-tab("Attacks on my Server: The Data", "https://cliffle.com/blog/data-on-attacks/")

== Rustonomicon
+ #link-new-tab("Atomics", "https://doc.rust-lang.org/nomicon/atomics.html")
+ #link-new-tab("Implementing Vec", "https://doc.rust-lang.org/nomicon/vec/vec.html")
+ #link-new-tab("Implementing Arc and Mutex", "https://doc.rust-lang.org/nomicon/arc-mutex/arc-and-mutex.html")
+ #link-new-tab("Uninitialized Memory", "https://doc.rust-lang.org/nomicon/uninitialized.html")

== Adrian Sampson
+ #link-new-tab("Flattening ASTs (and Other Compiler Data Structures)", "https://www.cs.cornell.edu/~asampson/blog/flattening.html#fn:inline")
+ #link-new-tab("Bril: An Intermediate Language for Teaching Compilers", "https://www.cs.cornell.edu/~asampson/blog/bril.html")
+ #link-new-tab("Manual Test-Case Reduction", "https://www.cs.cornell.edu/~asampson/blog/reduction.html")
+ #link-new-tab("FPGAs Have the Wrong Abstraction", "https://www.cs.cornell.edu/~asampson/blog/fpgaabstraction.html")
+ #link-new-tab("Is JavaScript Statically or Dynamically Scoped?", "https://www.cs.cornell.edu/~asampson/blog/scope.html")

== Verdagon (Evan Ovadia)
+ #link-new-tab("Borrow checking, RC, GC, and the Eleven (!) Other Memory Safety Approaches", "https://verdagon.dev/grimoire/grimoire")
+ #link-new-tab("Higher RAII, and the Seven Arcane Uses of Linear Types", "https://verdagon.dev/blog/higher-raii-uses-linear-types")
+ #link-new-tab("The Impossible Optimization, and the Metaprogramming To Achieve It", "https://verdagon.dev/blog/impossible-optimization")
+ #link-new-tab("Group Borrowing: Zero-Cost Memory Safety with Fewer Restrictions", "https://verdagon.dev/blog/group-borrowing")
+ #link-new-tab("Exploring Seamless Rust Interop for Newer Languages, Part 1", "https://verdagon.dev/blog/exploring-seamless-rust-interop-part-1")
+ #link-new-tab("Vale's First Prototype for Immutable Region Borrowing", "https://verdagon.dev/blog/first-regions-prototype")
+ #link-new-tab("Hash Codes, Non-Determinism, and Other Eldritch Horrors", "https://verdagon.dev/blog/generics-hash-codes-horrors")

== Faster than a Lime
+ #link-new-tab("Introducing facet: Reflection for Rust", "https://www.youtube.com/watch?v=0mqFCqw_XvI&t=427s")
+ #link-new-tab("The virtue of unsynn", "https://www.youtube.com/watch?v=YtbUzIQw-so&t=1s")
+ #link-new-tab("strace feels like magic — let’s fix that (with Rust)", "https://www.youtube.com/watch?v=engduNoI6DE&t=757s")

== Oxide
+ #link-new-tab("Futurelock", "https://rfd.shared.oxide.computer/rfd/0609")
+ Helios: #link-new-tab("post", "https://rfd.shared.oxide.computer/rfd/0026"), #link-new-tab("video", "https://www.youtube.com/channel/UCFn4S3OexFT9YhxJ8GWdUYQ")
+ #link-new-tab("Dealing with cancel safety in async Rust", "https://rfd.shared.oxide.computer/rfd/0400")
+ #link-new-tab("Holistic Boot", "https://rfd.shared.oxide.computer/rfd/0241")
+ Propolis
+ TODO add other Oxide & Friends


== Blargh (Thomas Habets)
+ #link-new-tab("io_uring, kTLS and Rust for zero syscall HTTPS server", "https://blog.habets.se/2025/04/io-uring-ktls-and-rust-for-zero-syscall-https-server.html")


== Hillel Wayne 
+ #link-new-tab("10 Most(ly dead) Influential Programming Languages", "https://www.hillelwayne.com/post/influential-dead-languages/")
+ #link-new-tab("Planner programming blows my mind", "https://www.hillelwayne.com/post/picat/")

== Mond
+ #link-new-tab("Type Inference in Rust and C++", "https://herecomesthemoon.net/2025/01/type-inference-in-rust-and-cpp/")

== Sunshowers (Rain)
+ #link-new-tab("Debugging a rustc segfault on illumos", "https://sunshowers.io/posts/rustc-segfault-illumos/")
+ #link-new-tab("Rain's Rust CLI recommendations", "https://rust-cli-recommendations.sunshowers.io/")
+ #link-new-tab("Lifetime variance in Rust", "https://lifetime-variance.sunshowers.io/") (combine with #link-new-tab("Rustonomicon: Subtyping and Variance", "https://doc.rust-lang.org/nomicon/subtyping.html"))
+ #link-new-tab("The social consequences of type systems", "https://sunshowers.io/posts/types/")
+ #link-new-tab("Example: implementing Borrow for complex keys", "https://github.com/sunshowers-code/borrow-complex-key-example")


== Ginger Bill
+ #link-new-tab("On the Aesthetics of the Syntax of Declarations", "https://www.gingerbill.org/article/2018/03/12/on-the-aesthetics-of-the-syntax-of-declarations/")
== Sebastian Bensusan
+ #link-new-tab("We need visual programming. No, not like that.", "https://blog.sbensu.com/posts/demand-for-visual-programming/")


== Fernando Borretti
+ #link-new-tab("Introducing Austral: A Systems Language with Linear Types and Capabilities", "https://borretti.me/article/introducing-austral")

== Chris Siebenmann
+ #link-new-tab("I don't think error handling is a solved problem in language design", "https://utcc.utoronto.ca/~cks/space/blog/programming/ErrorHandlingNotSolvedProblem")

== Polybdenum
+ #link-new-tab("Why You Need Subtyping", "https://blog.polybdenum.com/2025/03/26/why-you-need-subtyping.html")

== Ulrich Drepper
+ #link-new-tab("What Every Programmer Should Know About Memory", "https://people.freebsd.org/~lstewart/articles/cpumemory.pdf") (long)

== Yorick Peterse
+ #link-new-tab("The inevitability of the borrow checker", "https://yorickpeterse.com/articles/the-inevitability-of-the-borrow-checker/")


== Celes
+ #link-new-tab("The Language That Never Was", "https://blog.celes42.com/the_language_that_never_was.html")

== Ryan Brewer
+ #link-new-tab("Par Part 1: Sequent Calculus", "https://ryanbrewer.dev/posts/sequent-calculus")
+ #link-new-tab("Par Part 2: Linear Logic", "https://ryanbrewer.dev/posts/linear-logic")

== Phantie
+ #link-new-tab("Inventing a Better Compression Algorithm for a Specific Problem", "https://phantie.dev/articles/inventing_a_better_compression_algorithm_for_a_specific_problem?category=engineering")

== Mike Davis
+ #link-new-tab("Sparse arrays in Rust and creating custom DSTs.", "https://the-mikedavis.github.io/posts/sparse-array-dst/#fighting-for-a-dst-a-fool-s-errand")

== Conviva
+ #link-new-tab("The Concurrency Trap: How An Atomic Counter Stalled A Pipeline", "https://www.conviva.ai/resource/the-concurrency-trap-how-an-atomic-counter-stalled-a-pipeline/")


= Other Topics
- DTrace
- eBPF
- Async architectures (cooperative multitasking, work stealing, etc.) & Tokio specifically
- LLVM
- OS
- VMs
- PL topics (lexers, parsers, ..)
- Hashing
- Maybe also do some basic topics?

#html.hr()

