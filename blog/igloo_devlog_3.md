---
title: Igloo Devlog 3
desc: Finalized ECS, Floe Protocol, ESPHome, & Queries
date: 2025-10-05
homepage: true
---

[Project Page](https://liamsnow.com/projects/igloo)

The past month has been an insane amount of progress on Igloo.
I went from only having a rough plan to having a fully working
ESPHome Floe, finalized ECS model, query system, and a lot of
the server code implemented. While I certainly have a lot more
to do (Frontend/UI + Penguin), I am extremely proud of what I
have accomplished.

# Table of Contents
<!--toc:start-->
- [Numbers](#numbers)
- [Floe Protocol](#floe-protocol)
  - [Putting the ECS Model In](#putting-the-ecs-model-in)
    - [Attempt 1: Giant Enum](#attempt-1-giant-enum)
    - [Attempt 2: One Component Per Command](#attempt-2-one-component-per-command)
    - [Attempt 3: Transactions](#attempt-3-transactions)
  - [Payload Serialization](#payload-serialization)
- [Code Generation](#code-generation)
- [Glacier](#glacier)
  - [ECS Representation](#ecs-representation)
    - [Entities](#entities)
    - [Devices](#devices)
  - [Device Tree](#device-tree)
  - [Further Optimizations](#further-optimizations)
- [Query System](#query-system)
- [What's Next](#whats-next)
<!--toc:end-->


# Numbers
Before getting into all the details, I want to show some
of the numbers I'm most proud of:
 1. $60 \mu s$ to turn off every ESPHome light in my apartment
 2. $\approx 15 n s$ to serialize an entity update
 3. 3.4MB RAM usage with 10 ESPHome devices connected

# Floe Protocol
**Goals**:
 1. Multi-language support
 2. Backwards-compatible
 3. Ergonomic for developers



[Protocol Buffers](https://protobuf.dev/) would be the obvious choice
for a lot of people, because it generates code in just about every language
from a single spec file. BUT my experience with using it in ESPHome it honestly
no great. I find the generated code to be usable, but not very ergonomic.

The next obvious option is using JSON. This means I still have to generate types
in other languages, and it's hard to always get them to play nice. I figure
if I am going to go through the process of code-generation, might as well
have a binary protocol.

Taking inspiration from ESPHome's I decided to go with a similar wire format:

```
[len: varu32] [cmd_id: varu16] [payload...]
```

This leverages variable length integers. It's a really cool spec
you can read about more [here](https://sqlite.org/src4/doc/trunk/www/varint.wiki)
but basically it small numbers (<240) get encoded as 1 byte, while bigger
numbers get encoded with more bytes.
 - An `Int` component here gets encoded as $6$ bytes!!

Under this system we can easily implemented backwards-compatibility by:
 1. Never change the payload of commands, if you want new functionality add a new command
 2. If you see a `cmd_id` you don't know, skip it based off the `length` of the packet


## Putting the ECS Model In
### Attempt 1: Giant Enum
My initial thought was to represent the Components in a giant enum:

```rust
enum Component {
  Int(i32),
  Uint(u32),
  // ...
}
```

Sending updates would then require giving a device ID, entity ID, and a `Vec<Component>`.
While this system is great for Rust, it doesn't really map over to other languages
so nicely. Rust enums are just ahead of their time.

### Attempt 2: One Component Per Command
Instead, I decided to go with a more decentralized approach by never representing
them in a grouped manner.

This mean dropping batching and making a command each for component.
This means your sending device ID, entity ID, and 1 components value.

Its effective and relatively sending a few extra bytes wont hurt anyone.
But it was super annoying to work with while making the ESPHome Floe.

In the ESPHome Floe I really need to update all components of an entity at
once (to reduce \# of commands sent over TCP).

Sure, I could keep of track of the last entity IDs and batch it all together,
but I just wasn't really happy with that.

### Attempt 3: Transactions
The final version I came up with relies on transactions,
basically it looks like this:

```
StartTransaction { DEVICE }
SelectEntity { ENTITY }
Int 10
Text "example"
DeselectEntity
EndTransaction
```

This gives us a massive benefit on both sides. On the server side,
as soon as they select an entity we can grab a mutable reference
to the entity they select and never have to do multiple lookups.


## Payload Serialization
Since all our payloads are pretty simple (either single value primitive, enum, or struct of the same)
we can go with a pretty simple solution.

 1. [MessagePack](https://msgpack.org/index.html): well supported in many languages, but usually requires manually writing parsing code
 2. [CBOR](https://cbor.io/)
 3. **[Borsh](https://borsh.io/)**: really fast and stable multi-language format
 4. [bincode](https://crates.io/crates/bincode) really fast, no multi-language support really

Borsh is **super** cool. In my test cases it can serialize transactions
in around 15ns (bincode @ 30ns, JSON @ 350ns). Furthermore, its
great because it doesn't require `serde` which helps a ton with
compile times.



# Code Generation

In the last dev log I alluded to generating components using a [Procedural Macro](https://doc.rust-lang.org/reference/procedural-macros.html)
(`#[component]`) , but it didn't really make since because
we are generating code for multiple languages and want one common spec.

I decided to model components in a [Toml](https://toml.io/en/) file and then use
a [Build Script](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
to generate Rust, Python, and Golang source code from that.

Here's what it looks like:
```toml
[[components]]
name = "Int"
id = 0 # NEVER change this ID or versions will be incompatible
derive_bound_types = [1, 2, 3] # makes IntMin (id=1), IntMax (id=2), IntStep (id=3)
derive_inner_list_type = 4 # makes IntList, id=4
kind = "single"
field = "i32"
desc = "signed 32-bit integer"

# ...

[[components]]
name = "Time"
id = 34
derive_list_type = 35
kind = "struct"
fields = [
    { name = "hour", type = "u8" },
    { name = "minute", type = "u8" },
    { name = "second", type = "u8" }
]

# ...
```

This then generates Rust, Python, and Golang code:
 - Structs & Enums
 - Command IDs
 - Command serialization methods

It also generates Rust server code with:
 - Giant Component and ComponentType enums
 - `Averageable` trait implementations automatically
 - Protocol <-> Component interop


# Glacier

Glacier (name may be changed later) is the system I made to manage Floes and global state.

## ECS Representation
### Entities
I needed a way to represent entities such that:
 1. Entities can only have 1 component of each type (IE can't have 2 `Int`s)
 1. It's fast to check if an entity has a component

Here's what I came up with:

```rust
#[derive(Debug)]
pub struct Entity {
    /// stores the actual components
    components: SmallVec<[Component; 8]>,
    /// maps Component ID -> index in `.components`
    /// 0xFF = not present
    indices: [u8; MAX_SUPPORTED_COMPONENT as usize],
}
```

This is extremely fast for a few reasons:
 1. **Component Exists:** single array check (Type ID is index, exists is `!= 0xFF`)
 2. **Component Value:** array + smallvec lookup
 3. Most entities live directly in the struct, no hopping around 
    - I decided that most entities have `<=8` components
    - [smallvec](https://crates.io/crates/smallvec) allocates space for `8` components directly in the struct, but expands with pointers
    - Basically for most of entities, its extremely fast, for big entities its _slightly_ slower 
 4. Fixed-Size Array for `.indices` means the compiler can optimize much more than on a `Vec`

### Devices
To make checks even faster, we also keep track of presense of components on Devices.
This means if were looking for a `Light` and a device doesn't have any, we don't
have to iterate over all its entities:

```rust
#[derive(Debug, Default)]
struct Device {
    entities: SmallVec<[Entity; 16]>,
    presense: [u32; MAX_SUPPORTED_COMPONENT.div_ceil(32) as usize],
}
```

## Device Tree
Initially I planned for Glacier to own the entire Device Tree, then threads
would forward Floe's commands over `mpsc` channels to Glacier.

Under this system a normal update happens like this:

```
Hardware Device -> LAN -> Floe (device manager) -> Floe (main) -> Unix Socket -> Glacier (floe manager) -> mpsc channel -> Glacier (main) -> Query Reciever
```

While this system works perfectly fine, I actually realized there's no reason to 
represent the device tree together. We can simply have each Floe manager thread
track their own data. Then we just dispatch queries to each one of them.

Cutting out this step is a huge simplification and reduces the delay for getting updates:

```
Hardware Device -> LAN -> Floe (device manager) -> Floe (main) -> Unix Socket -> Glacier (floe manager) -> Query Reciever
```

Now the main Glacier system just keeps track of zones and device names.

## Further Optimizations

We need to keep persistent data about zones and device names. Devices are registered
under an initial name, but then the user might want to change that name - we need this
persistence.

I decided to use UUIDv7 (prevents overlaps since devices are never registered at the same time)
expanded with the Floe name. For example `ESPHome-0199a2c3-690b-74ba-acd6-3e1da15857af`.

While this is great for preventing collisions, having to do a ton of these hashmap lookups is slow.

So, I came up with a new system. Devices are registered with their persistent UUID
and a text name for all of their entities. This data gets transferred to the main Glacier
system.

BUT Now updates between the Glacier Floe manager and Floe used indicies. IE simply use device `0`
entity `5` to refer to the 5th entity registered under the first device registered by that Floe.
It simple and fast.


# Query System
The next big question in the ECS model is how do we read and write components?
I really like the [query system that Bevy has](https://bevy-cheatbook.github.io/programming/queries.html),
so I modeled mine after that.

All queries take 2 things:
 1. A filter: filter which entities apply based on a set of rules (ex. only entities with Light component)
 2. An area: filter down to only a specific Zone, Device, or Entity

Then we have a few types of queries:
 1. Set: change the value of components
 2. Get: get the values of all components of a certain type (ex. get the value of all Dimmers)
 3. Average: average the values of all components of a certain type
 4. WatchGet: register a persistent query and receive updates when any of them change
 5. WatchAvg: ^^
 6. Snapshot: for the rare case where you _do_ want a full picture

Now we have a super powerful system. For example, this turns off all Lights in the kitchen:

```ron
Query(
  filter: With(Light),
  area: Zone("Kitchen"),
  kind: Set([Switch(true)]),
)
```

Just for fun I decided to benchmark query execution time
and found that I can turn off all the lights in my house in $60 \mu s$
(time between query dispatch and being sent out over Unix socket
for ESPHome Floe).


# What's Next
The core of the backend is mostly there, now I just have
to start working on the Frontend + Dashboard system
and see all of this come to life!!

