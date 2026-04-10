#metadata((
  title: "Cache Coherence",
  desc: "Why you should care about cache coherence protocols.",
  written: "2026-03-30",
  updated: "2026-03-30",
  homepage: true,
)) <page>

#import "../../_shared/template.typ": post, link, link-new-tab 
#show: post

= Why should I care?
Let's say we have a mutex, and `N` threads constantly trying to read from it. 

Your intuition probably says that regardless of `N`, the throughput
should be pretty similar (the mutex essentially serializes the threads).
And you'd be right for any `N>1`, the throughput is about the same.

But I'll bet you wouldn't have guessed that for `N=1` we get \~10x the throughput.
By understanding your hardware (and specifically the MESI cache coherence protocol), you'll understand _why_.


= What do cache coherence protocols solve?
On most multicore machines, all cores:
 - Have their own local caches
 - Share the same RAM

Let's imagine a basic world without _cache coherence protocols_:
 + Core \#1 reads `x=24`, then caches it
 + Core \#2 reads `x=24`, then caches it
 + Core \#1 writes `x=32` to cache & RAM (\* write-through is not standard, covered later)
 + Now Core \#2's cache holds an invalid value!


= Introduction

Hardware has changed a lot, which in turn, changed the requirements and capabilities of cache coherence protocols.
For the purposes of understanding modern computers, you really only need to know that CPUs have an interconnect, enabling them to communicate with each other (peer-to-peer) and with RAM (via an #link-new-tab("integrated memory controller (IMC)", "https://en.wikipedia.org/wiki/Memory_controller")).
Furthermore, caching *does not* operate on every possible RAM address, but instead on cache lines -- commonly a 64 byte chunk.

While all protocols have the same requirement (provide cache coherence), there is a lot more nuance to them.
Different designs can yield better performance, usually by reducing communication (core-to-RAM and core-to-core).


#html.details()[
  #html.summary()[More history]
== Hardware
=== Bus-Based (Legacy)
On older machines (1990s & 2000s), all cores shared a single #link-new-tab("front-side bus (FSB)", "https://en.wikipedia.org/wiki/Front-side_bus").
The bus connected to a #link-new-tab("northbridge", "https://en.wikipedia.org/wiki/Northbridge_(computing)"), which only then actually talked to the RAM:

#html.elem("img", attrs: (
  src: "cache_coherence/northbridge.svg",
  width: "300px"
))

Given this architecture, the bus had to be mutually exclusive (only controlled by 1 core at a time).
While this does make it a bottleneck, it also makes coordination simple: 
cores can just "snoop" on the bus to know when to invalidate (or even directly update) their cache.


=== Modern On-Die Interconnect
Modern CPUs have neither a northbridge nor an FSB, instead:
 - an #link-new-tab("on-die interconnect", "https://en.wikipedia.org/wiki/Intel_QuickPath_Interconnect") (note other CPUs have \~ equivalent things)
 - an on-die, #link-new-tab("integrated memory controller (IMC)", "https://en.wikipedia.org/wiki/Memory_controller") (which is on this interconnect)

Unlike FSB, the interconnect facilitates peer-to-peer communication (it is not a broadcast). 
This difference has a huge effect on coherence strategies.


== Basic Cache Coherence Protocols
We have two basic cache coherence protocols for bus-based architectures:
+ *Write-Update*: When a core writes to a #link-new-tab("cache line", "https://en.algorithmica.org/hpc/cpu-cache/cache-lines/"), it broadcasts the new value, which other cores use to update their cache.
+ *Write-Invalidate*: When a core writes to a cache line, it broadcasts a message telling other cores to invalidate their copy.

Write-update may initially seem faster -- cores don't have to refetch data.
But there's more nuance, and in practice, write-invalidate uses significantly less bandwidth. Take this example:
 - If core \#1 writes 10x and core \#2 reads it after the writes
 - Write-update had \#2 update the cache 10x 
 - Write-invalidate had \#2 invalidate once, and refetch once

Both have room for improvement:
 - The bus is a bottleneck -- all coherence traffic and memory requests compete for the same mutually exclusive resource
 - We're sending invalidation messages even when no other cache holds the line (wasted bandwidth)
 - These protocols assume write-through (every write goes to both cache and RAM). Write-back caches are more efficient (less memory traffic), but they make coherence harder — now main memory can be stale too, so the protocol needs to track which lines are dirty.

== Modern Coherence

On the interconnect, broadcast is expensive but targeted peer-to-peer communication is cheap.
This creates entirely different constraints for the cache coherence protocols.

Many use MESI (covered below) or a variant like MOESI or MESIF.
Alongside, they use a *directory-based* approach:
 - A directory tracks exactly which caches hold each line
 - When a core needs to invalidate or fetch, the directory tells it who to talk to
]

= The MESI Protocol

MESI is a cache coherence protocol found in many machines.
In it, *each core* store a small amount of state for *every cache line*.

== States

#table(
  columns: 2,
  [*State*], [*Meaning*],

  [#strong[M]odifed],
  [Dirty / differs from RAM, only this core has it],

  [#strong[E]xclusive],
  [Clean / matches RAM, only this core has it ],

  [#strong[S]hared],
  [Clean / matches RAM, multiple cores have it],

  [#strong[I]nvalid],
  [This core doesn't have it (or the cached value is stale)],
)

== Operations
=== Read
*Line is `Modified`, `Exclusive`, or `Shared`* (Hit):
 - Simply return the cached value (no state change or bus activity)

*Line is `Invalid`* (Miss). Next step depends on what other cores hold:
+ *None*: retrieve from RAM, cache as `Exclusive`
+ *`Exclusive`*: retrieve from that core, both cores → `Shared`
+ *`Modified`*: retrieve from that core, both cores → `Shared`, write-back to RAM (clean it)
+ *`Shared`*: retrieve from any core, cache as `Shared`

=== Write
*Line is `Modified`* (Hit): just modify cache

*Line is `Exclusive`* (Hit): modify cache, → `Modified` (mark dirty)

*Line is `Shared`* (Hit): tell others → `Invalid`, modify cache, → `Modified` 

*Line is `Invalid`* (Miss). Next step depends on what other cores hold:
+ *None*: retrieve from RAM, modify, → `Modified`
+ *`Exclusive` or `Shared`*: tell others → `Invalid`, retrieve from RAM, modify, → `Modified`
+ *`Modified`*: tell other to write-back to RAM & → `Invalid`, retrieve from RAM, modify, → `Modified`

#html.details()[
  #html.summary()[Why the Exclusive State?]

MESI added the exclusive `Exclusive` state to MSI, which reduced traffic:
 - If a core is the sole owner and the line is clean, it can promote to `Modified` on a write with zero bus activity
 - In MSI, a line held by only one core is still marked `Shared`, so every write requires broadcasting an invalidate -- even when nobody else has the line
]


= Putting it Together
Let's go back to our example from the start --
why is it so much slower with `N>1` than `N=1`?

In Rust, locking a mutex involves an atomic operation on shared state:
```rust
pub fn lock(&self) -> MutexGuard {
  // state is shared data
  // this (atomic) operation tries to acquire the lock
  while self.state.compare_exchange(0, 1, ...).is_err() {
    // spin or park
  }
}
```

Now that we understand MESI, we know that:
 - If `N=1`, that core can keep the cache line in `Modified`. The `fetch_add` simply modifies its local cache, without ever touching RAM or communicating over the bus
 - If `N>1`, we'll have a ton of bus traffic -- cores constantly taking the line as `Modified` and invalidating others

Aha!

= Footnotes
This was my first post going into depth about topic outside my projects. I'd love to hear what you think! 

Jon Gjengset's, "#link-new-tab("The Cost of Concurrency Coordination", "https://www.youtube.com/watch?v=tND-wBBZ8RY")" talk was the inspiration for this post. I'd highly recommend giving it a listen or any of his other videos.

Additionally, I referenced (and would recommend checking out!) these sources:
+ #link-new-tab("Cache coherence in shared-memory architectures (University of Texas)", "https://www.cs.utexas.edu/~pingali/CS377P/2018sp/lectures/mesi.pdf")
+ #link-new-tab("The MESI protocol (University of Pittsburgh)", "https://people.cs.pitt.edu/~melhem/courses/2410p/ch5-4.pdf")
+ #link-new-tab("MESI Protocol Wikipedia", "https://en.wikipedia.org/wiki/MESI_protocol")

