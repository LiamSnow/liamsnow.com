---
title: Igloo Devlog 2
desc: Planning
date: 2025-09-20
homepage: true
---

[Project Page](https://liamsnow.com/projects/igloo)



I have spent the last month working on planning Igloo in more detail and confirming
or changing some of my past decisions. Spending extra time on this planning stage
is very important for me.

As much as I love working on the GUI for Penguin, I realized that I need to focus
on flushing out the backend server first.

# Table of Contents
<!--toc:start-->
- [New Terminology](#new-terminology)
- [Table of Contents](#table-of-contents)
- [ECS-Style Device Tree](#ecs-style-device-tree)
  - [Objectives](#objectives)
  - [Prototype 1: Standards (not ECS)](#prototype-1-standards-not-ecs)
  - [Prototype 2: Strongly Typed (not ECS)](#prototype-2-strongly-typed-not-ecs)
  - [Prototype 3: Bevy-like ECS (Selected Version)](#prototype-3-bevy-like-ecs-selected-version)
- [Provider Support](#provider-support)
  - [Linux Style](#linux-style)
  - [Package Style](#package-style)
  - [Final Decision](#final-decision)
- [Floes](#floes)
- [Data/State Management](#datastate-management)
- [Penguin in Bevy](#penguin-in-bevy)
- [Next Steps](#next-steps)
<!--toc:end-->

# New Terminology
 1. Packages/extensions for Igloo are called **Floes** to match the theme.
 2. The central management system for the Device Tree and Floes is called **Glacier** also to match theme.

# ECS-Style Device Tree

Given that the device model in Home Assistant is one of my biggest gripes, I wanted
to really make sure I have good system for this. I played around with doing it many ways.

## Objectives
 1. **Backwards-compatible**: New versions of Igloo must be able to work with old Floes
 2. **Structured but Flexible**: We need to have clearly defined types to create a cohesive system. I don't want a situation where ESPHome has one light type and HomeKit has another, meaning we need different dashboard elements and Penguin nodes for each. BUT I want to have the flexibility to support both:
   - New types of devices that aren't yet built into Igloo
   - Extensions to existing types - maybe a new provider has a Fan with 10 different modes but Igloo only has a few
 3. **Intuitive**: Understandable for the average contributor
 4. **Cross-language**: While Rust will be the focus for this interface, having support for other languages is a must. Protobuf would be nice to generate Python and Rust types, but worst comes to worst we at least need to be able to write an untyped Python provider.
 5. **Composable**: We need a way to group things with similar functionality. For example, maybe I want to control the color of everything that can colored (Light Bulbs, LED strips, etc.)

## Prototype 1: Standards (not ECS)
In this model, devices have a component tree that can abide by standards.

Components:
```rust
pub enum Component {
    Int(i32),
    Float(f64),
    Long(i128),
    String(String),
    Bool(bool),
    // ...
    Object(HashMap<String, Component>),
    List(Vec<Component>),
}
```

Then we have a set of standards. For example, `Light` says that
it must have `Bool` called `on`, and can optionally have `brightness`, `color_temperature`,
and `color`:

```rust
impl Components {
    pub fn conforms(&self, std: Standard) -> bool {
        use Type::*;
        match std {
            Standard::Light => {
                self.has_req_comp_of("on", Bool)
                    && self.has_opt_comp_of("brightness", Float)
                    && self.has_opt_comp_of("color_temperature", Float)
                    && self.has_opt_comp_of("color", Color)
            }
           // ...
```

This is quite an interesting composition-based style. It actually follows all the objectives:
 - Its backwards compatible. New versions would simply just add optional components
 - Its pretty structured (could be more) but also flexible (providers can just add more components onto a Light before the standard is updated)
 - Its pretty understate

However, it has some downsides:
 1. Cumbersome for provider maintainers: it requires them to go and read up on all the standards, what properties they expect and such. Basically, not have LSP support is quite annoying.
 2. It puts burden on provider maintainers to make sure they inherit all standards. For example, for every Light they would need to specifically that it's a `Light`, `Dimmable`, `Colorable`, `Switchable`, and potentially more. It would be very easy for maintainers to forget to add all of these.
 3. Strange validation step. What if they marked it as a `Light`, but it doesn't have `"on"`.
 4. Prone to mistakes: what if they marked it as a `Light`, but spelled it `"brighness"` instead of `"brightness"`. The system would validate it fine, but it wouldn't work.

Overall I just wasn't really happy with this system.


## Prototype 2: Strongly Typed (not ECS)
Devices contains a set of components which either fully follow a strong type or are untyped.

Example Device Tree:
```ron
{
  "ceiling_light": Device(
    provider: String("ESPHome"),
    perms: Inherit,
    components: {
      "RGBCT_Bulb": Light(
        on: true,
        brightness: 1.0,
        color: Color(r: 255, g: 0, b: 0),
      ),
      "Status": Bool(true),
      "Safe Mode": Bool(false),
      "Uptime Sensor": LongSensor(
        unit: "seconds",
        value: 12310927398,            
      ),
      "IP Address": String("192.168.1.201"),
      "Mac Address": String("..."),
      "Connected SSID": String("..."),
      "Firmware": String("..."),
    }
  )
}
```

This system wins in some key areas:
 1. Super easy to work with a provider maintainer. Everything is a Rust struct, meaning they LSP autocomplete for creating things
 2. No validation steps, it just works
 3. Pretty composable, but not amazing. For example:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    #[serde(flatten)]
    pub switchable: Switchable,
    #[serde(flatten)]
    pub colorable: Colorable,
    #[serde(flatten)]
    pub dimmable: Dimmable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Switchable {
    pub on: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimmable {
    pub brightness: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Colorable {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

But falls short in more, and especially violates the objectives:
 1. Inflexible: Either you exactly follow the Light standard or you have to make an untyped Object. That really sucks for maintainers and makes them have to wait for a PR to be merged and make sure all users update their system to work with new features.
 2. I'm not actually totally sure how to even implement this system in JSON which is basically a requirement for cross-language support. I'm sure I could figure out it, but it wouldn't be very clean.

## Prototype 3: Bevy-like ECS (Selected Version)
 - A Device represents a single hardware device, is a collection of entities 
 - Entities are a collection of Components
 - Components either store value/type or are markers (similar to Bevy)
 - Components can require other Components

Example Device Tree:
```json5
{
  "ceiling_light": {
    "provider": "ESPHome",
    "perms": "Inherit",
    "entities": {
      "RGBCT_Bulb": [
        {
          "Light": null // just a marker
        },
        {
          "Switch": {
            "value": true
          }
        },
        {
          "Dimmer": {
            "value": 1.0
          }
        },
        {
          "Color": {
            "value": {
              "r": 255,
              "g": 0,
              "b": 0
            }
          }
        }
      ],
      "Status": [
        {
          "Bool": {
            "value": true
          }
        }
      ],
      "Safe Mode": [
        {
          "Bool": {
            "value": false
          }
        }
      ],
      "Uptime Sensor": [
        {
          "LongSensor": {
            "unit": "seconds",
            "value": 128231289
          }
        }
      ],
      "IP Address": [
        {
          "String": {
            "value": "192.168.1.201"
          }
        }
      ],
      "Mac Address": [
        {
          "String": {
            "value": "..."
          }
        }
      ],
      "Connected SSID": [
        {
          "String": {
            "value": "..."
          }
        }
      ],
      "Firmware": [
        {
          "String": {
            "value": "..."
          }
        }
      ]
    }
  }
}
```

Interface Code:
```rust
#[derive(Component)]
#[require(Switch)]
struct Light;

#[derive(Component)]
struct Switch {
  value: bool
}

#[derive(Component)]
struct Dimmer {
  value: f64
}

#[derive(Component)]
struct Color {
  r: u8,
  g: u8,
  b: u8,
}
```

I really like system. I feel like the implementation is very clean and it's easy to understand.

It follows all of our set objectives:
 1. Its backwards compatible: Because components represent such a small thing, it's rare for them to be changed. We can easily add new components that can be slowly adopted. It really gives the user and maintainers full power. For example, they can easily make a Dashboard element that looks for `With<Light, ColorTemperature>`
 2. It's structured with hard-coded types, but its also extremely flexible as mentioned above.
 3. Cross-language support should be pretty straight forward, we can simply generate docs or directly generate Python from the Rust source code. The JSON is easy enough were we really don't need Python types.
 4. Composable and intuitive as mentioned


# Provider Support

There's a few ways I could have taken supporting providers and extensions (IE adding Penguin nodes, Dashboard elements, etc.):

## Linux Style
Build everything into a giant monorepo.

Pros:
 - Faster: direct intra-process communications (probably `tokio`'s `mpsc`)
 - Probably smaller total size: Instead of having 20 Rust binaries all with their own `tokio`, you just get one compact optimized binary.
 - Simple: No interface at all, no package management system

Cons:
 - Updates to 1 provider require full restarts
 - Slow build times: has to compile the whole system (could be optimized with flags for providers, Rust libraries, etc.)
 - Fragile: 1 provider memory leak would probably take out the whole system
 - No Python Support: probably fixable by using Pyo3 or maybe having a package system for only Python versions, but that seems like a bit much
 - No Containment: everything ones as one process with the same permissions. If somehow a rogue maintainer edits their provider, they can hack everyone's Igloo

## Package Style
Pros:
 - Containment: providers can be run with specific permissions
 - Multi-language support (probably just Rust and Python)
 - Easily update providers without restarts
 - Fast build times and development for both Igloo and individual providers
 - Easy updates: we can easily update Igloo and if a provider requires extra work to update theirs, the whole system is backward-compatible

Cons:
 - Slower: realistically not a huge concern, most of the delay is IO-bound
 - Have to build package system and interface: definitely some extra work, but very doable

## Final Decision
Looking at the pros and cons I think its pretty clear why I am choosing to stick with
the Package style. It wins out for many reasons, but especially Python support - something
that a smart home community thrives on.

# Floes
After decided to stick with Package style, I wanted to figure out how to it right.

My current plan is to have a file structure like this:
```bash
igloo # main binary 
floes/
  FLOE/
   BINARY
   Floe.toml # describing file, like Cargo.toml
   ... # other files for the Floe
  ...
...
```

`Floe.toml` would describe authors, name, version, config for it as a provider, which Penguin nodes
it has, what Dashboard elements it adds, etc.

Python Floes would work slightly different, instead of having a binary it would simply have a
`requirements.txt` or `pyproject.toml` and Igloo would run a main python file with the packages
installed.

I was playing around with the idea
of compiling Floes into WASM so it works on any machine + has a ton of extra security, but
not being able to have hardware access is kind of a deal breaker for me. I think the best
way to go about this is having people upload a cargo project to my website and have my
server compile it for Raspi + x86.

# Data/State Management

The prototype for Igloo was basically ephemeral except for saving tokens in a SQLite database.

Since this version doesn't live around a config file system, saving state is required. I was playing
around with using SQLite for this, and while I do think it would work totally fine, most of my data
doesn't really need SQL queries. Furthermore, the device tree system I cam up with doesn't really work
well with SQL.

I decided that simply having everything in memory (as Rust types) and then backing up to file would simply
work better. We get better performance and if we loose some entity state updates from a crash, its totally
fine. I have a prototype on the [GitHub](https://github.com/LiamSnow/igloo/tree/e70d9d18fec26362b52d53faf9d0529ec7342a07/server/src/auth).


# Penguin in Bevy

As I mentioned in my last post, I had a prototype using Dioxus. While this system worked
pretty good, it had a lot of problems and extra complexity. I know HTML-based Node editors
exist and work well, writing it in something like Bevy gives me a ton more tools, performance,
and just overall a cleaner architecture. I have a prototype on the [GitHub](https://github.com/LiamSnow/igloo/tree/e70d9d18fec26362b52d53faf9d0529ec7342a07/frontend/src/editor).

# Next Steps
 1. Write code for the new Device Tree/interface
 2. Write server code (glacier)
 3. Implement basic provider Floes for testing
