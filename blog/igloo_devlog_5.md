---
title: Pushing Igloo's query engine to the limit
desc: How Igloo is able to reach 1 million queries per second
date: 2025-11-24
homepage: true
---

This article assumes you are familiar with both Igloo's ECS system and device tree. If you are not I would recommend you to check out my past blog posts on these.
If you don't know what Igloo is please check out the [Project Page](../projects/igloo).

The query engine is giant project and I cannot cover all of it here.
I would recommend you to check out the code which is linked throughout this article.

# What
Create a system to get data from the device tree.
It needs to get data about extensions, groups, and devices
(including their entities and components).

Common queries:
 1. Continuously watch the average brightness of lights in the kitchen
 2. Turn off all lights in living room

Types of Queries:
 1. __One-Shot__: Evaluate once
 2. __Observer__: Persistent/continuous


# Goals
The overarching goal of the query engine is to be fast and simple.
We are __not__ creating a SQL database.

Filters and operations have a few goals:
 1. QOL: Create filters that are nice to have and useful for many use cases
 2. Make observers powerful. This is important because:
    1. If filters are not powerful, you would have to constantly be unregistering and registering new observers when new devices/entities/components are added
    2. We don't want users to have to constantly filter observer updates

# Why Make It Fast
Obviously no Wi-Fi light can respond to commands at 1MHz.
Furthermore, the bulk of the latency for setting and getting status updates from Wi-Fi devices is IO-bound. So, why make it fast? Wouldn't 1ms be reasonable?

To address this problem you really need to understand that Igloo is centered around the device tree.
If you want to do just about anything in Igloo, it involves the device tree, and by extension, the query engine.
Making the query engine faster, makes everything faster.
It makes scripts execute faster, the UI snappier, and can actually reduce resource utilage.

A super important point of this too, is that I am trying to reach 1 million queries per second on my desktop computer (i9-10900k, 10G networking, 4GHz RAM) in my (currently simple) smart home. When I go to put this on a Raspberry Pi 3, it will be substantially slower.

# The Model
[The query model](https://github.com/LiamSnow/igloo/blob/main/interface/src/query/model.rs)
lives inside igloo interface, a shared crate. It allows for 5 targets of queries:
Extension, Group, Device, Entity, and Component.

The most important type of query, and what we'll focus on, is the component query.
Component queries are very powerful, allowing for the following actions:
 1. Get value (one-shot)
 2. Set: sets value of existing component on entity
 3. Put: put a component onto an entity OR set existing value
 4. Apply: modify value of existing component (ex. multiply by 2)
 5. Count: counts number of components that matched filters
 6. Observe value: same as get value

Component queries can have 2 main types of filters:
 1. Device Filter: by ID, device's owning extension, group, entity count, last update
 2. Entity Filter: by ID, what other components it has, values of other components it has, last_update

Finally, component queries can have a post/aggregation operation (mean, min, etc.)

# Optimizing Queries
In initial versions, the [query optimizer](https://github.com/LiamSnow/igloo/blob/main/interface/src/query/optimize.rs)
had a strong role.
Now that the query model is very strict and structured, it doesn't need to be very complex.
It handles deduplication, reordering queries to evaluate cheap operations first,
and flattening.

# Evaluating One-Shot Queries
To avoid making this article unnecessarily long, I will walk through only how one-shot get value aggregated component queries work (ex. get average value of lights in the kitchen).

We need to:
 1. iterate over devices that match the filters
 2. iterate over their entities that match the filters
 3. pull out the value of the component we are looking for
 4. apply aggregation to all values we collected

# Optimizing One-Shot Queries

[Query Engine Source](https://github.com/LiamSnow/igloo/tree/main/server/src/query)

## Reducing Search Set
The biggest optimization is to do everything we possibly can to not have to scan
every single entity in ever single device. This will __always__ be slow.

Luckily, our various filters + bidirectional device tree mappings provide many opportunities for this. For example:
 1. Device filter says devices only of IDs 0,1,2 → Only iterate over those devices
 2. Device filter says only devices in group 5 → Lookup group, find device IDs in group, iterate over those

Furthermore, we have entity search set optimizations. Each device contains a mapping
from `ComponentType` → entity index. If the query is looking for component `Dimmer`,
we plug in dimmer and get a list of entity indexes to iterate over.

## Iterators → Closure Generics
We have various types of component queries as specified above,
so we really don't want to have reimplement the device & entity filtering code for each.

The most Rust idiomatic way to do this is by using iterators.
Simply make an `iter_entities` method which takes in a device & entity filter. 
Then you can use this iterate to do whatever else on it (collect values in this case).

The problem with this is you end up having an iterator at least 3 chains.
Normally rustc is good at optimizing this, but it struggles in this case.

Through some testing, I found that closure generics were substantially faster.
Here's an example:
```rust
#[inline]
pub fn for_each_entity<F>(
    ctx: &mut QueryContext,
    tree: &DeviceTree,
    device_filter: &DeviceFilter,
    entity_filter: &EntityFilter,
    mut f: F,
) -> ControlFlow<()>
where
    F: FnMut(&Device, &Entity) -> ControlFlow<()>,
```

This approach will inline and create a new `for_each_entity` function for each different component query. 

## Avoiding Collection With Code Generation
My initially version would collect all the components into a `Vec<_>`,
then apply the aggregation operation. However, we don't actually need to do
this if we don't allow for op::median (which isn't very useful in smart homes anyways).

Now, we can construct an aggregator and push values to it. For the case of op::mean,
this simply means counting the number of entries and running sum. Then we call
finish, it divides them:

```rust
let Some(mut agg) = Aggregator::new(query.component, op) else {
    return Ok(Err(QueryError::InvalidAggregation(query.component, op)));
};

let _ = for_each_entity(
    &mut self.ctx,
    tree,
    &query.device_filter,
    &query.entity_filter,
    |_, entity| {
        if let Some(comp) = entity.get(query.component) {
            agg.push(comp)?;
        }
        ControlFlow::Continue(())
    },
);

R::Aggregate(agg.finish())
```

This extensively leverages code generation:
```rust
impl Aggregator {
    pub fn new(comp_type: ComponentType, op: AggregationOp) -> Option<Self> {
        match (comp_type, op) {
            (ComponentType::Integer, AggregationOp::Sum) => {
                Some(Aggregator::IntegerSum { sum: 0 })
            }
            (ComponentType::Integer, AggregationOp::Mean) => {
                Some(Aggregator::IntegerMean { sum: 0, count: 0 })
            }
            // ... for every valid ComponentType, AggregationOp combination
        }
   }
}
```

This provides massively improves performance by not having to allocate a vector, push to it, and then iterate over.

# Channels
With these optimizations and many others, I can evaluate the
example query (get average brightness of lights in the kitchen) on my desktop PC
for my smart home in a mean of **220ns**! I am super happy with these results.

You're probably sitting their thinking, wait that's almost 5 million queries per second!
Sadly, the query engine can't just have hard coded queries and sit their spinning them as fast as possible. We need a system for clients to ask for queries to be evaluated and receive the results of them.

In the system I made, clients register with the engine (via its channel) and provide a channel
where they want responses send back to. I initially decided to use Tokio's mpsc channel.

I built a test case that spawns up some fake clients that all hammer the query engine
with requests. I tried with a different number of clients and different frequencies
of query requests. In all cases, I was not able to get past 150k queries per second.

Researching and experimenting with different channels (
[crossbeam](https://crates.io/crates/crossbeam),
[flume](https://crates.io/crates/flume), and
[kanal](https://crates.io/crates/kanal)),
I found that kanal outperformed every other library for this use case
(mpsc for request, spsc for responses, both bounded channel of N size holding a lot of data).
With kanal, it can achieve ~1.2 million queries per second throughput.

# What's Next
Now that the core functionality of Igloo server is working,
I am going to start working on the dashboard system! 

At some point I will come back to revisit the query engine
when we add temporal data. This is a very important feature
for smart homes to have. For example, it's very common to want
to see the history of the temperature in a room.
