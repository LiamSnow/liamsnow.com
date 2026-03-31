#metadata((
  title: "Bringing ECS to smart homes",
  desc: "Why ECS is a good abstraction for smart home devices",
  written: "2025-08-27",
  updated: "2026-03-30",
  links: (
    ("Project Page", "/projects/igloo"),
    ("Homepage", "https://igloo.rs"),
    ("GitHub", "https://github.com/liamsnow/igloo"),
  ),
  homepage: true
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post

*Context*:
 - Igloo is a smart home platform like #link-new-tab("Home Assistant", "https://www.home-assistant.io/")
 - It should be able to work with any smart home device
 - Igloo extensions are separate processes that are effectively device drivers (bridging vendor-specific interfaces and Igloo)

= The Problem
We need a way to abstract away the nuances of the vendor-specific interfaces to their devices. Without this, we'd break the cohesion of Igloo.

*Ex.* Let's say a user has 4 lights in their kitchen, all from different vendors. The user must be able simply turn off all the lights together, without ever thinking about the different vendor-specific interfaces.

== Goals for the Abstraction
We have a few goals for the abstraction
 - Some goals fight against each other -- it's a balancing game

*Exposes Nuances*:
 - Devices shouldn't be handicapped. Maybe some lights support a transition time, and others support effects -- all of these should be exposed
 - New types of devices should be fit in

*Cohesive*: See example above

*Easy to Use*: It shouldn't be extremely complicated to use

*Polyglot*: Must work well in at least Rust & Python, and ideally also Go & TypeScript

= Case Studies
== Home Assistant
In Home Assistant, a device is broken up into entities.
Each entity has a name and follows a strict structure.
Currently, there are
#link-new-tab("47 entity types", "https://developers.home-assistant.io/docs/core/entity/light/"),
with everything from a light to a lawn mower.

*Ex.* A #link-new-tab("Athom Color Bulb", "https://www.athom.tech/blank-1/15w-color-bulb-for-esphome") contains many entities:
 - `"RGBCT_Light"`: light
 - `"Status"`: string
 - `"IP Address"`: string
 - `"MAC Address"`: string
 - `"Uptime Sensor"`: sensor
 - ...

*Pros*: Cohesive, easy to work with, fits perfectly into any language

*Cons*: Hides nuances until (and if) the definition is updated
 - New types of devices can't be added until respective definition(s) are
 - Handicaps devices that have functionality outside definition
 - Sometimes doesn't map cleanly to vendor specific interfaces

== OpenHAB
#link-new-tab("OpenHAB", "https://www.openhab.org/") takes a more
#link-new-tab("flexible approach", "https://www.openhab.org/docs/configuration/things.html") which is quite interesting.

It aligns much more with my goals (notably not hiding nuance),
but I think that ECS just handles it better.


= ECS Model
If you're outside the game-development world, you may have never heard of the
#link("https://en.wikipedia.org/wiki/Entity_component_system")[ECS (Entity-Component-System)] model.
It's a powerful, composable architecture where you have:
 - *Entity*: set/group of components
    - Each entity can only contain 1 of each component (it's a set)
 - *Component*: a small & primitive value (or ZST/tag)
    - In games, you'll normally see `Position` and `Velocity`
 - *System*: logic that operates on components
    - Instead of having specific functions to move every game object, you'd make a tick-based system that simply applies the `Velocity` to the `Position` (although its probably more complex than that) 
    - Commonly, systems use a #link-new-tab("\"query\" functionality", "https://bevy-cheatbook.github.io/programming/queries.html") -- a mechanism to specify what entities the system targets (what components the entity must have or must not have)

In a smart home, a light entity could be:
 - `(Switch(false))`
 - `(Switch(false), Dimmer(0.5))`
 - `(Switch(false), Dimmer(0.5), Color(..))`
 - or include any other nuance

To aid with queries, tags/ZST are useful. By adding a `Light` component to a light entity, queries can be more explicit. Notably, the use of that component in a query is entirely optional (up to the user).

This system achieves our goals:
 - *Cohesive*: a system could simply look for entities with `Switch` and turn them off
 - *Exposes Nuance*: nuance is easy via composing any combination of components
 - *Easy to Use*: while harder than Home Assistant's model, it's relatively easy to understand and work with (via queries)
 - *Polyglot*: by all means this system is polyglot, however, it works best if targeting only Rust

Additionally:
 - We don't have to constantly update the definition for a light, and in many cases, we might not even have to add new components for new devices
 - If we keep components as what they should be (small & primitive), we can simply make them immutable (making the defined components append-only). Doing this makes the system easily backwards-compatible (IE Igloo could interact with older extensions)
 - No need for definitions with many optional fields -- if a light doesn't accept a color, it just won't have the component

== A Complete Example

This is what our #link-new-tab("Athom Color Bulb", "https://www.athom.tech/blank-1/15w-color-bulb-for-esphome") looks like in this:
 - *Note*: I've given entities names here which is not standard ECS. This can simply be replaced via a `Name(String)` component.

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
"WiFi Signal": [
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
Because this needs to be polyglot, I've opted to with code generation from a TOML file. Rust #link-new-tab("build scripts", "https://doc.rust-lang.org/cargo/reference/build-scripts.html") will take this TOML and output both Rust & Python typings.

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

=== Values
Given that we are supporting Python, I've decided to limit the types.
For example, we only have 1 integer type and not all possible Rust ints.

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

= Conclusion & Post-Mortem
I was unhappy with existing smart home device abstractions,
and decided to look for a new one.
#link-new-tab("Bevy", "https://bevy.org/") inspired me to apply ECS here, and it worked out well.

I don't think ECS is perfect -- it's not the most ergonomic, can be unintuitive for new users, and is not the easiest to implement. But, I think it's the least bad.

As I've been working through this project, I've constantly questioned if it's the right abstraction, and explored other options. But I keep coming back to it -- which is why I'm confident it's the best choice here.

