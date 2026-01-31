#metadata((
  title: "Bringing ECS to smart homes",
  desc: "Igloo's device model",
  date: "2025-08-27",
  homepage: true
)) <page>

#import "/_shared/template.typ": post
#show: post

If you don't know what Igloo is please check out the #link("../../projects/igloo")[Project Page].

There are a lot of ways to represent devices and their state in smart homes.
Through my research and experimentation, I landed on using the ECS model, which
I think is a great system for Igloo.

Before diving into it, I want to walk you through how I came to this conclusion.

= Objectives
First, I outlined my objectives:
 + *Backwards-compatible*: New versions of Igloo must be able to work with old Extensions
 + *Structured but Flexible*: We need to have clearly defined types to create a cohesive system. I don't want a situation where ESPHome has one light type and HomeKit has another, meaning we need different dashboard elements and Penguin nodes for each. At the same time, I want to have the flexibility to support both:
     + New types of devices that aren't yet built into Igloo
     + Extensions to existing types - maybe a new provider has a Fan with 10 different modes, but Igloo only has 8
 + *Intuitive*: Understandable for the average _contributor_
 + *Cross-language*: While Rust will have primary support, having support for at least Python is essential.
 + *Composable*: We need a way to group things with similar functionality. For example, maybe I want to control the color of everything that can be colored (Light Bulbs, LED strips, etc.)

= How Home Assistant Does It
Devices contain many entities. For example, my Athom light bulb contains a RGBCT_Light entity,
a safe mode (switch) entity, etc.

At the time of writing this, #link("https://developers.home-assistant.io/docs/core/entity/light/")[Home Assistant has 47 hard-coded models]
for each type of entity (light, lawn mower, media player, etc.).
This makes a lot of sense. You get a strict structure for how to represent things,
but also you get flexibility with optional parameters for each entity.
However, it comes with some downsides:
 - Must define and keep 3 models in sync: storage struct, command enum, and status update enum
 - New types of devices cannot be represented
 - May not align well with other providers

= ECS Model
If you're outside the game-development world, you may have never heard of the
#link("https://en.wikipedia.org/wiki/Entity_component_system")[ECS (Entity-Component-System)] model.
It's a powerful, composable architecture.
 - *Entity*: Simply a grouping of components. Each entity can have 1 of each component.
 - *Component*: A model for a very specific thing. In game development this might be Position3D.
 - *System*: The business-logic that works on components

While using an ECS model for a smart home initially sounds like a strange concept,
it actually makes a lot of sense for our goals.
We can very easily design it to be both structured and very flexible,
representing potentially an infinite amount of different devices.
It has some great benefits:
 - We don't need to explicitly define each type of entity
 - No need for many optional fields, if a light doesn't accept a color, it simply won't have that component
 - Easily backwards-compatible: We enforce an append-only schema where components are never changed or deleted, only added. This provides forward compatibility, old systems simply ignore new components they don't understand.

== What This Looks Like

This is a representation of an Athom RGBCT ESPHome Light Bulb in Igloo's ECS model:

```rust
"Status": [
  Sensor, Diagnostic, DeviceClass("connectivity"), Boolean(true)
],
"RGBCT_Bulb": [
  Light,
  Color(IglooColor { r: 1.0, g: 1.0, b: 1.0 }),
  Dimmer(1.0),
  Switch(true),
  ColorTemperature(2000),
  ColorMode(Temperature)
],
"Uptime Sensor": [
  Sensor, Diagnostic, SensorStateClass(TotalIncreasing),
  Icon("mdi:timer-outline"), DeviceClass("duration"),
  Unit(Seconds), AccuracyDecimals(0), Real(54.01900100708008)
],
"${friendly_name} WiFi Signal": [
  Sensor, Diagnostic, SensorStateClass(Measurement),
  DeviceClass("signal_strength"), Unit(DecibelsMilliwatt),
  AccuracyDecimals(0), Real(-57.0)
],
"Reset": [
  Config, Icon("mdi:restart-alert"), DeviceClass("restart")
],
"Safe Mode": [
  Config, Icon("mdi:restart-alert"), DeviceClass("restart")
],
...
```

== Implementation
To implement this ECS model I decided to go with `build.rs` code generation
in a shared crate called `igloo-interface`. This interface crate will
be used by both the core system, frontend, extensions, and scripts.

Code generation allows us to easily generate Rust and Python code from a TOML file.
In `igloo-interface` we have a file called `components.toml`:

```toml
[[components]]
name = "Weekday"
id = 17
kind = "Enum"
variants = [
    { id = 0, name = "Sunday" },
    { id = 1, name = "Monday" },
    { id = 2, name = "Tuesday" },
    { id = 3, name = "Wednesday" },
    { id = 4, name = "Thursday" },
    { id = 5, name = "Friday" },
    { id = 6, name = "Saturday" },
]

[[components]]
name = "Light"
id = 18

[[components]]
name = "Switch"
id = 19
kind = "Boolean"

[[components]]
name = "Dimmer"
id = 20
kind = "Real"
desc = "Range: 0.0 (0%) - 1.0 (100%)"
```

=== Component Types
 - *Single Value*: This is most components, just store 1 value (like `Dimmer`)
 - *Enum*: Generate a custom enum (like `Weekday`)
 - *Marker*: Marker components (also called tag components) are zero-sized types that contain no data. They're used purely for filtering and queries. The `Light` marker is a perfect example: you can query for all entities with both `Light` and `Dimmer` components to find all dimmable lights in the system.

=== Values
I decided to limit the number of types we have in Components.
This small subset simplifies FFI with Python and other languages by avoiding complex types that don't map cleanly across language boundaries.

```rust
type IglooInteger = i64;
type IglooReal = f64;
type IglooText = String;
type IglooBoolean = bool;

struct IglooColor {
    r: f64,
    g: f64,
    b: f64,
}

struct IglooDate {
    year: u16,
    month: u8,
    day: u8,
}

struct IglooTime {
    hour: u8,
    minute: u8,
    second: u8,
}

type IglooIntegerList = Vec<i64>;
type IglooRealList = Vec<f64>;
type IglooTextList = Vec<String>;
type IglooBooleanList = Vec<bool>;
type IglooColorList = Vec<IglooColor>;
type IglooDateList = Vec<IglooDate>;
type IglooTimeList = Vec<IglooTime>;
```
