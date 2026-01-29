#import "../../shared/template.typ": post
#show: post.with(
    base: "blog",
    routes: toml("../routes.toml"),
    filename: "igloo/prototype.typ"
)

If you don't know what Igloo is please check out the #link("../projects/igloo")[Project Page].

Initially I set out with a very different goal for Igloo.
I wanted a smart home platform for power users that was reliable and fast.
I knew this would be a big project, but it turned out to be much bigger than I thought.

Overall, I would call this version a failure. However, it was a great learning experience.

= What I Built

The prototype used an immutable configuration approach inspired by NixOS:

*Architecture:*
- Zones contain Devices contain Entities (similar to Home Assistant's model)
- Two interfaces: REST API and WebSocket
- CLI-based protocol for both interfaces:
  - `list devices kitchen`
  - `light all off`
  - `light kitchen.sink off`
- Everything configured in a single Ron file: users, permissions, provider configs, devices, zones, UI elements, and scripts
- Immutable and ephemeral: No data is saved between runs, nothing is ever changed once running. The config file is the center of truth.

*Tech Stack:*
- Backend: Rust with Tokio and Axum
- Frontend: SolidJS

The CLI protocol was elegant for scripting, and the configuration-as-code approach felt clean on paper.

#image("prototype.png")

= What I Learned

== Scale and Complexity

This was my first substantial Tokio project. Managing providers, devices, entities, channels, concurrent tasks, dashboards, frontend, authentication, API, and scripting simultaneously was more complex than I anticipated.

Making providers is harder than I thought. The #link("../projects/esphomebridge-rs")[ESPHome provider] alone took a lot of time, especially because it was completely undocumented.

== Configuration File is a Pain

The NixOS-inspired configuration file seemed like a good approach,
but was tedious and had little benefits. Users had to:
- Generate password hashes manually with `igloo hash PASSWD`
- Edit a Ron file for all configuration changes
- Restart the system to add or remove devices
- Understand the configuration file structure before using the system

While I do think this system improves upon Home Assistant's YAML system,
I think the configuration file is always going to be a pain point.
It requires users to spend a significant amount of time browsing documentation
just to get Igloo working.

== Architectural Problems

The prototype used hard-coded entity types, which created cascading complexity. For each entity type, I needed to define:
- A struct to store the entity's state
- An enum for commands (requests to the entity)
- An enum for state updates from the entity

This meant adding a new entity type required changes throughout the codebase. Worse, providers were built directly into Igloo like Linux kernel drivers, resulting in slow compilation times and tight coupling between providers and the core system.

== The CLI Protocol Trade-off

The CLI-based protocol (`light kitchen off`) was clean for scripting but was limiting and again
requires users to learn a new system and read through documentation.
A good GUI can provide autocomplete, visual feedback, and discovery which a CLI could never match.
While the CLI protocol worked well for automation scripts, it was the wrong primary interface for users.

= V2 Direction

For my WPI Major Qualifying Project this year, I'm doing a complete rewrite that addresses these lessons:

+ *UI-based configuration*: All setup happens through the interface. The layout will be tested with users to ensure good UX.

+ *Visual programming for automations*: Home Assistant demonstrated that many users prefer visual interfaces for creating automations over YAML configuration. A full visual programming language will provide more power while remaining accessible. Users and providers can create custom nodes using Python or other languages.

+ *Flexible architecture*: Moving away from hard-coded entity types to reduce coupling and complexity.

+ *Standard provider interface*: Providers will be separate from the core system, allowing them to be written in any reasonable language without slow compilation or tight coupling.
