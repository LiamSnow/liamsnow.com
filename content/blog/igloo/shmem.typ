#metadata((
  title: "Distributed Device Tree & New IPC",
  desc: "Shared memory, ring buffers, atomics, seqlocks, & more",
  written: "2026-02-23",
  updated: "2026-02-23",
  links: (
    ("Igloo", "/projects/igloo"),
  ),
  homepage: true
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post

*Context*:
 - Igloo is a smart home platform like #link-new-tab("Home Assistant", "https://www.home-assistant.io/"), connecting any smart home device into a cohesive system
 - Igloo extensions are effectively device drivers (bridging vendor-specific interfaces and Igloo)
 - Igloo currently runs extensions as separate processes #link("providers")[(see post)]
 - Igloo abstracts devices using the ECS model #link("ecs")[(see post)]

= Background
The current IPC is 1 Unix socket between the core and each extension.
This is not only a massive bottleneck (serializing device communication),
but also means each extension needs to route commands.
IE:
 + An extension gets a request over the socket
 + Looks up device in table, finds spsc channel
 + Sends request through spsc channel
 + Device's Tokio task wakes up, dispatches over TCP

This bottleneck also applies for the reverse directory (extension committing
change to the core).

This problem can be somewhat mitigated by splitting the TCP socket
into read and write halves or by passing around the file-descriptor.
This means core → ext can remove the spsc channel entirely.

Ultimately this solution works fine and is faster than anyone
would notice (most delay is IO-bound). Mostly for fun, I wanted
to explore an option that would give us better speed and remove
the bottleneck.

= Ring Buffers
Shared memory means both the core and the extension can see this memory.
We can have the core create shared memory and send the file-descriptor
over a Unix socket for the extension to see. 

A very common use of shared memory is ring buffers.
We'd basically make two shared memory spsc (single-producer single-consumer)
ring buffers.

We could even scale this further to have 2 ring buffers per device (tx, rx),
meaning the core could talk directly to each device's Tokio task.

== Notifications
There's a slight hiccup here and that's a notification system.
We need a way for the extension and core to know when to check their rx buffers.

Luckily, this is solved problem and has a few answers:
 + *Futex*: reader does a `FUTEX_WAIT` on a count and writer does `FUTEX_WAKE` after putting something new in.
   - This does not integrate with Tokio's #link-new-tab("epoll", "https://en.wikipedia.org/wiki/Epoll") reactor at all
   - We could either drop async entirely (not really an option) or switch to a different runtime like #link-new-tab("monoio", "https://github.com/bytedance/monoio") or #link-new-tab("glommio", "https://github.com/DataDog/glommio") to use this.
 + #link-new-tab("eventfd", "https://man7.org/linux/man-pages/man2/eventfd.2.html"): create an eventfd for each ring buffer we make
   - Works with basically any async runtime
 + *Interval polling*: Simply check it every N ms
   - No syscalls, works with every runtime
   - Trade fast response time for more CPU usage
 + *Spinlock*: Just crank CPU reading from it
   - Completely burns a core
   - No syscalls, fastest response time

== Conclusion
While this can solve the bottleneck, it's not entirely fair to say that.
We _could_ also just make a Unix socket for every device.
So the only real benefit here is faster communication.

A nice part is that this has been done and in Rust (#link-new-tab("shmem-ipc", "https://github.com/diwic/shmem-ipc")).

= Distributing the Device Tree
In the current model, even if we replace Unix sockets with ring buffers,
we'll always have a bottle neck -- the main thread.
The main thread owns the device tree. This means any read queries, write queries, or commits from extensions MUST flow through it.

So let's get rid of it!
 + Distribute metadata (attached devices, names, groups, etc):
    + We want to optimize for the read-heavy workload. Sequential locks could work well here, but Rust also has a lot of good options (like #link-new-tab("papaya", "https://lib.rs/crates/papaya")).
 + Pack components into `AtomicU64`'s. Lock-free reads and writes.
   - The atomic operations _will_ be slower. However, the penalty is a lot less than sending a query over an mpsc channel. 
 + For bigger components (strings and vectors) we implement #link-new-tab("seqlocks", "https://docs.kernel.org/locking/seqlock.html"):
   - Write:
      + `Relaxed` store `seqlock` to odd
      + `Release` fence (so seq stored before write)
      + Write new Data
      + `Release` store `seqlock` to even (so data is written fully before seq store)
   - Read:
      + `Acquire` load `seqlock` (so seq load is before comp read)
         - if odd, retry (write in progress)
      + Read data
      + `Acquire` fence (so comp is read fully before second seq load)
      + `Relaxed` load `seqlock`
         - if changed, retry
   - This gives us lock-free reads and writes. The cost is that readers _may_ have to re-read. However, in practice at our frequencies this is extraordinarily rare.

This not only reduces latency, it:
+ increases throughput
+ balances CPU load across cores
+ removes a lot of moving parts

== Comparison Examples
=== ESPHome Commit
*Old*:
+ Physical device changes ---tcp-→
+ ESPHome Device Tokio Task ---mpsc-→ 
+ ESPHome Main Tokio Task ---unix socket-→
+ Core Tokio Task for ESPHome ---mpsc-→
+ Core Main Thread

*Per-Device Unix Socket*:
+ Physical device changes ---tcp-→
+ ESPHome Device Tokio Task ---socket-→ 
+ Core Tokio Task for Device ---mpsc-→
+ Core Main Thread (writes to tree)

*Distributed Device Tree*:
+ Physical device changes ---tcp-→
+ ESPHome Device Tokio Task ---socket-→ 
+ Core Tokio Task for Device (direct write to tree)

=== ESPHome Request
*Old*:
+ User clicks button on dashboard ---web socket-→
+ Tokio Task for Client ---mpsc-→
+ Core Main Thread (evals) ---mpsc-→
+ Core Tokio Task for ESPHome ---unix socket-→ 
+ ESPHome Main Tokio Task ---mpsc-→
+ ESPHome Device Tokio Task ---tcp-→ 
+ Physical device

*Per-Device Unix Socket*:
+ User clicks button on dashboard ---web socket-→
+ Tokio Task for Client ---mpsc-→
+ Core Main Thread (evals) ---mpsc-→
  - \*still proxy to not slow down main thread
+ Core Tokio Task for ESPHome ---unix socket-→ 
+ ESPHome Device Tokio Task ---tcp-→ 
+ Physical device

*Distributed Device Tree*:
+ User clicks button on dashboard ---web socket-→
+ Tokio Task for Client ---unix socket-→
   - task is able to eval query directly on shared device tree
   - we avoid risk of clients writing to unix socket at the same time via `SOCK_SEQPACKET` (msg boundaries preserved, each send it atomic)
+ ESPHome Device Tokio Task ---tcp-→ 
+ Physical device

= Column-Wise Bitset Queries

The goal of queries is to really quickly evaluate "find the avg brightness of lights." This effectively means "find the average value of Dimmer components that are on entities with a Light ZST component."

Commonly, these are scoped to either a device or group of devices.

The idea here is having a 2d array of bits (an array of bitsets) that represent if an entity has a given component. This gives us a superpower, we can simply do `presense[Dimmer] & presense[Light]` or `presense[Dimmer] & .. & group_scope[id of kitchen]`.

What we get back is a bitset which correlates to slot indexes (in this case slots are entities stored globally) which matched our query. We can iterate this and grab all our values via `values[Dimmer][slot ID]`.

It's branch-less, cache-friendly (dense), no pointer chasing, and, once we have the result, fast to iterate over.

This approach actually works great with the distributed device tree. I implemented a #link-new-tab("POC", "https://github.com/LiamSnow/igloo-device-tree-testing") and achieved much better performance than previous versions (1 billion QPS compared to 1 million). It's also just simpler.


= Device Tree in Shared Memory
In designing the distributed device tree, we made a concurrent data structure that has a fixed-size for each device. This almost perfectly maps into shared memory (shmem).

In a shmem approach, the device tree would live in between providers and the core.
Each provider would still have a Unix socket, but only for high level management.

Providers use this socket to request to add a device. Then, the core maps this maps the memory and passes it back to the provider. The core then only maintains the metadata for the device tree, and reading state simply involves reading from the shared memory pages for devices. Providers can simply directly write to this memory.

The downside of this approach is the watcher mechanism. In the old approach, we'd compile watchers into "event listeners" and trigger them based off Unix socket messages. Here, we have no idea when a provider wrote to the shared memory. As we discussed earlier, futexes don't integrate into Tokio, leaving us really only with eventfd.

While we _could_ make an eventfd for every entity or even every components, we quickly flood the amount of file descriptors we have open. The most realistic option is to have an eventfd for each device, but that's quite course. If the eventfd got triggered, we'd either need a complex dirty flag system (which somehow supports multiple readers) or require each watcher to compare diffs. Neither solution is great -- all add complexity and cause unnecessary wakes.

Furthermore, we just deleted our request mechanism. We have a few options, but none seem great:
 - Use the provider Unix socket, but this defeats the entire point
 - Make a Unix socket for each device, but then what's the point of shmem?
 - Duplicate the device tree (have one for requesting), but now we have the same problems as the last paragraph mentions

I spent a long time trying to make this work, but I think that ultimately it's a ton of complexity and huge error surface, just to shave off a few microseconds.

= Conclusion
This was a great exploration. While not all paths were successful, two were (distributed device tree & column-wise bitset queries). We created a simpler and faster system, that has almost no bottlenecks. And I was able to prove this with the #link-new-tab("POC", "https://github.com/LiamSnow/igloo-device-tree-testing").
