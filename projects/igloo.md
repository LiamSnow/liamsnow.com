---
title: igloo
desc: A secure, fast, & intuitive smart home platform
date: 2025-05-06
homepage: true
---

[GitHub Repo](https://github.com/liamsnow/igloo)

[Website](https://igloo.rs)

# Background

[Home Assistant](https://www.home-assistant.io/) is a smart home platform that can
connect nearly any smart home product. It breaks down vendor lock-in and allows you
to have one singular platform to manage your entire home. On top of this, it allows
for scripting, custom dashboards, and automations. It makes smart homes fun and powerful.

Home Assistant is the reason I got interested in smart homes. I think it's an amazing
tool, but at the same time it has a lot of flaws. I understand why many of these flaws
exist. Put simply, smart homes are complicated and trying to integrate every smart home
product is a really hard goal.

While the Home Assistant developers and community are working hard to improve it, I think
the real solution is a complete rewrite and re-thinking of how it works. This is why
I am building Igloo.

# Goals
 1. **Intuitive**: The majority of people should be able to figure out how to navigate around Igloo's website and use existing dashboards without the need to watch tutorials. Creating dashboards, scripts, and setting up Igloo will most likely require tutorials, but they should not need to be extremely long or complicated.
 2. **Robust & Reliable**: The system must be able to run, untouched, and not crash itself. Furthermore, we should never have massive breaking updates that requires users to go around fixing scripts and configurations.
 3. **Fast**: Commands, API-requests, and the dashboard should all feel snappy and fast. 
 4. **Secure**: It must be safe to expose to the public internet without any risks of RCE, unauthorized access, or leaked secrets.
 5. **Run "Anywhere"**: It must be able to run a small smart home on a Raspberry Pi 3.

# Development

More to come! See my [Blog](../blog) for development updates.
