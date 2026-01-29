#import "../../shared/template.typ": post
#show: post.with(
    base: "blog",
    routes: toml("../routes.toml"),
    filename: "igloo/device_tree.typ"
)

If you don't know what Igloo is please check out the #link("../projects/igloo")[Project Page].

#link("https://github.com/LiamSnow/igloo/tree/main/server/src/tree")[GitHub Link]

= What
Igloo's device tree needs to represent:
 + Devices, their entities, and their components
 + Device Groups
 + Extensions

Most things in the device tree are ephemeral, but some things need to persist:
 + Devices: ID, Name, Owner (extension)
 + Groups: ID, Name, Devices (by ID)

= Device & Group IDs
== Persistent UUID, Ephemeral Index
Initially, I decided on a system where devices and groups have a persistent UUIDv7 ID. However, looking up devices by UUIDv7 in a FxHashMap is slower than direct array indexing. More importantly, using UUIDs increases the size of persistent data. Every reference to a device requires storing a 128-bit UUID instead of a compact index.

I decided to optimize this by maintaining devices and groups in a `Vec<_>`, with lookup tables mapping UUIDs to ephemeral indices. While this worked (as proved by Igloo V1), it added complexity with multiple moving parts.

== Generational Index
The final approach uses a generational index (also called generational arena) pattern. This system has one ID that always references by index. To handle device deletions safely, we represent devices as `Vec<Option<Device>>`, where `None` represents deleted slots. Since deletions are rare and creations are common, this vector stays densely packed in practice.

This creates a problem. If a script references device index 5, that device gets deleted, and a new device reuses index 5, the old reference appears valid but points to the wrong device. Generational indices solve this by versioning each slot.

Each device gets a generation assigned to it. Every reference (ID) to that device contains both the index and the generation. When a device at index 5, generation 0 (`5:0`) is deleted and a new device takes that slot, it gets generation 1. When the script tries to access `5:0`, it throws a stale reference error because the current generation is 1.

This system provides fast lookups (*>6x faster*), compact persistent storage, and memory safety.

= Modeling Entities
Since entities can only have one of each component type and entities contain a subset of all components, we can add significant optimizations.

We maintain an array the length of the number of component types in the system. Each item is an unsigned 8-bit integer mapping component type to its index in the components array, enabling O(1) reads and writes:

```rust
struct Entity {
    name: String,
    components: SmallVec<[Component; 8]>,
    indices: [u8; NUM_COMPONENTS],
}

impl Entity {
    pub fn get(&self, comp_type: ComponentType) -> Option<&Component> {
        let index = self.indices[comp_type as usize];
        if index != 0xFF {
            Some(&self.components[index as usize])
        } else {
            None
        }
    }

    pub fn put(&mut self, value: Component) {
        let type_id = value.get_type() as usize;
        let index = self.indices[type_id];

        if index == 0xFF {
            // put new
            let new_index = self.components.len() as u8;
            self.components.push(value);
            self.indices[type_id] = new_index;
        } else {
            // set existing
            self.components[index as usize] = value;
        }
    }
}
```

This is similar to dense component storage with an index lookup table. The `indices` array maps each component type to its position in the `components` SmallVec. We use `0xFF` (255) to indicate when a component type is not present on the entity. Since we index the SmallVec with u8, valid indices range from 0-254, making 255 available as a "not present" marker. This enables O(1) component lookups: check if `indices[type_id]` is `0xFF` to determine presence, and use it directly as an index otherwise.

The `SmallVec<[Component; 8]>` (from the #link("https://lib.rs/crates/smallvec")[smallvec] crate) provides additional optimization. SmallVec normally stores small arrays inline on the stack, but since `Entity` is heap-allocated inside `Device`, the small array is inline in the heap allocation rather than requiring a separate heap allocation. With typical entities having 4-8 components, this inline storage handles the common case without any heap allocation.

The array-based storage also provides better cache locality than pointer-chasing through heap allocations. The `indices` array fits in a cache line and component data is densely packed.

For hash maps where needed, we use `FxHashMap` from the #link("https://docs.rs/rustc-hash")[rustc-hash] crate. FxHash is a fast, non-cryptographic hash function that's significantly faster than Rust's default SipHash #link("https://nnethercote.github.io/perf-book/hashing.html")[(source)].

= Mutation System & Persistence
To ensure nothing in the device tree is improperly updated, I've placed the device tree in a module with everything set to module-only visibility. All mutations happen in a single file with extensive testing.

An important aspect of the mutation system is automatic persistence. Cold paths like create/rename/delete device automatically save to disk, whereas hot paths like component updates don't save because that data is ephemeral.

Persistent data is stored in INI files, which are simple, fast to parse, and human-readable without requiring #link("https://serde.rs/")[serde].
