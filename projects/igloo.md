---
title: igloo
desc: An intuitive smart home automation platform
date: 2025-05-06
homepage: true
---

[GitHub Repo](https://github.com/liamsnow/igloo)

## Why
I want to start out by saying that I think
[Home Assistant](https://www.home-assistant.io/)
is a groundbreaking tool and the most critical
part of any smart home. Its extensive device support
and active community are truly impressive.

However, that doesn't mean it comes without flaws:
 1. **Clunky and Poorly Designed UI**: the organization system of the UI is confusing. I find myself having to click into 6 different subpages for simple things. It sounds like a small problem but is consistently annoying. This also makes it really tedious to make automations, which I think is a core part of smart homes.
 2. **Feature Hell**: It seems like they just keep tacking on features that just make it all confusing. Having integrations, devices, entities, helper, areas, labels, zones, addons, dashboards, tags, and more is way overly complex. Furthermore, its common to add more to HASS by adding HACS so now you have another system.
 4. **Constantly Breaking**: Just leaving Home Assistant running on its own, it will eventually just break itself and require manual intervention.
 5. **Mixing of GUI and YAML Configuration**: The configuration system just seems like a mess of tech debt. Editing in the UI should be easy but doesnt give you all the features and is consistently buggy. Then when you try to edit in YAML is a mess of digging through documentation.
 6. **Slow**: Its crazy that for the workload in my house it basically requires that I have a Raspi 5 with a fan and heatsinks on it AND the website still loads slow.


## Igloo

With all of these pain points in mind, I decided to build
a competing smart home platform called Igloo.

I spent around six months making a prototype of Igloo and turns
out these smart home platforms are quite a lot harder than I thought.
There's a ton of big decisions to make all the time. It's really hard
to find a good architecture for this platform. But, its also really fun.

I'm really happy that I spent the time to make a prototype and just get
something working, because it taught me so much.

### The Prototype

In this prototype, I was really fixated on the idea of configuring it
like NixOS and easily writing scripts for it. However, looking back
now, if my goal is to make an intuitive smart home platform this is
definitely not the right approach.

Anyway, here's an executive summary of the Igloo prototype:
 1. Zones contain Devices contain Entities
 2. There is two interfaces to the system, API and websocket (used for the frontend for realtime updates)
 3. CLI-based protocol for both interfaces. Might seem weird but is actually a really clean way for working with a smart home. For example, `list devices kitchen`, `light all off`, `light kitchen.sink off`
 4. Everything is configured in a RON file (users, user groups, permissions, provider configs, devices and zones, ui elements, scripts). While it can definetly be a pain its nice that its all in one clear place.
   - This makes it immutable and ephemeral. To add or remove devices you must restart the program.
 5. Scripting is super easy with the use of the CLI protocol

I built it using Rust (Tokio + Axum) and SolidJS. I was pretty happy with this stack, but it was sometimes annoying having to copy interfaces into SolidJS.


#### Screenshot
![](/static/images/igloo_proto.png)



### Next Steps

This year I will have a lot of time to dedicate to Igloo as it will be a part of my MQP (Major Qualifying Project) at WPI. In this version I am planning a complete rewrite, learning from mistakes:
 1. Configuration is all UI based. The layout must be heavily thought out and tested with other people to verify it is intuitive.
 2. A visual programming language to allow users to easily create automations and scripts. People and providers can make their own nodes using Python or other languages.
 3. Intuitive architecture
 4. Standard provider interface such that providers can be easily written in any reasonable language
