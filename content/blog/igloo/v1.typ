#metadata((
  title: "Igloo V1",
  desc: "My first attempt at a smart home platform",
  written: "2025-03-14",
  updated: "2026-04-26",
  links: (
    ("GitHub (V1 Branch)", "https://github.com/LiamSnow/igloo/tree/v1"),
  ),
  homepage: false
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post

When I first started #link("/projects/igloo")[Igloo], I wanted to replace #link-new-tab("Home Assistant", "https://www.home-assistant.io/") with something much simpler and more reliable.
I was a bit naive coming into this, and it turned out to be a lot harder than I though.
Overall, I would call this version a failure, but I learned a lot, and knew how to approach the V2.

= Stack

The platform was built with Rust & Tokio.
It was my first substantial Tokio project, and I think some of the questionable architectural decisions reflect that 🫠.

The backend used Axum to expose a REST API and WebSocket for interaction.
This connected to a SolidJS frontend:

#image("prototype.png")


= Configuration

Everything was configured in a single immutable #link-new-tab("Ron", "https://github.com/ron-rs/ron") file (users, permissions, provider configs, devices, zones, UI elements, and scripts) which sounded great at first, but quickly became tedious.

Users had to:
- Generate password hashes manually with `igloo hash PASSWD`
- Edit a Ron file for all configuration changes
- Restart the system to add or remove devices
- Understand the configuration file structure before using the system

While I do think this system improves upon Home Assistant's YAML system,
I think the configuration file is always going to be a pain point.
It requires users to spend a significant amount of time browsing documentation
just to get Igloo working.

= Device Model

The device model/abstraction was heavily inspired by Home Assistant:
a device is represented by any number of entities, where each entity has a name, and follows a defined structure (Light, Switch, etc.).
I added an additional layer of hierarchy above devices called zones (usually for physical location/rooms).
This system works, but #link("/blog/igloo/model")[has a lot of room for improvement].

It used hard-coded entity types (as compared to code generation), which created cascading complexity. For each entity type, I needed to define:
- A struct to store the entity's state
- An enum for commands (requests to the entity)
- An enum for state updates from the entity


= Interface

The interface to the device model (API, WebSockets, automations) used a CLI-style.
It enabled multi-entity control (ex. controlling kitchen lights together) without needing explicit virtual entities/helpers.
This seemed like a cool idea, but only worked in simple cases, and put requirements on the naming of devices/zones/entities.
Examples:
```bash
list devices kitchen
light all off
light kitchen.sink off
```

= Device Drivers / Integrations
Igloo had a monolithic architecture, integrating device drivers directly into itself.
I only implemented #link("/projects/esphomebridge-rs")[1 driver] which took substantially more time than expected.

