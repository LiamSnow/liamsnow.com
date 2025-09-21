---
title: Igloo Devlog 1
desc: Penguin Work
date: 2025-08-27
homepage: true
---

[Project Page](https://liamsnow.com/projects/igloo)

# Table of Contents
<!--toc:start-->
- [Renaming](#renaming)
- [Penguin/Language](#penguinlanguage)
  - [Prototype 1](#prototype-1)
  - [Changing Course](#changing-course)
  - [Prototype 2](#prototype-2)
    - [Background](#background)
    - [Planning](#planning)
    - [Many Prototypes](#many-prototypes)
    - [Results](#results)
- [Next Steps](#next-steps)
<!--toc:end-->

# Renaming
After some though, I have decided to stick with the original project name Igloo (was Zaemon).
The programming/configuration language for it will be Penguin (was ZCL - Zaemon Configuration Language).

# Penguin/Language
## Prototype 1
I spent quite a long time thinking out how to make the Igloo configuration language.
I wanted something intuitive and powerful, which eventually led me down the path of
writing scripts directly in the configuration language.

I absolutely loved the experience of making an interpreted language in Rust, using LALRPOP grammar
and the Logos lexer. I made a pretty much fully functional language, you can check it out 
[here](https://github.com/LiamSnow/zcl).

As cool as this is, I could never find a style that intuitive for many people to understand.
Furthermore, I had concerns that expecting people to learn an entirely new programming language
just to configure their smart home may be daunting.

## Changing Course
I _really_ need to learn from Home Assistants lesson - YAML configuration
worked great at first, but quickly became overcomplicated, tedious, and less accessible to
many users.

After *a lot* of thinking, I ultimately decided that a visual programming is the best way to take this
project. Moving everything into a GUI allows for easier setup for users, device discovery, removes
scrolling through pages of documentation, and allows non-technical people to use it.

## Prototype 2
### Background
I have used a good amount of visual programming languages in the past.
Some are absolutely amazing ([Unreal Engine Blueprints](https://dev.epicgames.com/documentation/en-us/unreal-engine/blueprints-visual-scripting-in-unreal-engine),
[Blockly](https://developers.google.com/blockly))
and some are horrible ([LabVIEW](https://www.ni.com/en/shop/labview.html)).

There's certainly a lot more nuance to this, but I think the big differentiator between
good VPLs and bad ones, is simply how it feels to use the GUI. As dumb as it sounds, it
really makes a big impact. LabVIEW is extremely powerful, but I would never use it again
because of how annoying it is.

### Planning
I had big decision to pick from, I can either make a block-based VPL (IE Blockly, Scratch) or node based
(IE Unreal Engine Blueprints, LabVIEW, Blender Material Editor). Ultimately this is very preferential,
I think either would work well. I decided to go with node-based since I think it more clearly represents
an execution flow for automation. Part of this decision was also made because of the success of [n8n](https://n8n.io/)
which will be very similar to what I am making.

### Many Prototypes
There are a ton of different ways I could go about making this, but I knew I wanted to stick with Rust.

I made a bunch of super simple prototypes ([Bevy](https://bevy.org/), [WebGL](https://www.khronos.org/webgl/),
[WGPU](https://wgpu.rs/), and [Dioxus](https://dioxuslabs.com/)).
While the GPU-based versions (Bevy, WebGL, WGPU) are very cool (and faster), it requires a lot more
work on my part for relatively little benefit. I found that the Dioxus (basically React for Rust)
had good performance and was very easy to work with.

### Results
Right now, I have the Dioxus version working such that you can add, delete, select, drag, pan, zoom, and wire:

![](/static/images/penguin_prototype.png)

# Next Steps

Over the next few months I plan to:

 1. Finalize GUI
 2. Create saving system
 3. Create builtin + external node system
 4. Make interpreter
 5. Make an "LSP"


