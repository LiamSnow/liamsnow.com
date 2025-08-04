---
title: virtex
desc: 815fps FPGA-based IR machine vision camera
date: 2022-05-27
homepage: false
---

I decided I wanted to compete with [LimeLight](https://limelightvision.io/)
during COVID-19 so I designed a tiny camera that can
track retro-reflective targets at 815fps (!!) using FPGA-based vision
processing.

Crazy project to say the least. One day I'll get around to cleaning
up and releasing the devlog.

Sadly FIRST Robotics Competition switched to using April tags instead
of retro-reflective tape to I had to abandon the project.

## Sources
 - [Virtex HDL](https://github.com/34-Engineering/Virtex-HDL): Xilinx Vivado project for the Artix-7 (XC7A35T-1FTG256C) FPGA on Virtex
 - [Virtex PCBs](https://github.com/34-Engineering/Virtex-PCBs): Altium source for Virtex's PCBs and adapter board (34V0-CB-A, 34V0-CB-B, 34V0-CB-C, and 34AB0-CB)
 - [Virtex CAD](https://github.com/34-Engineering/Virtex-CAD): Fusion 360 CAD files for Virte and its adapter board (34V0 and 34AB0)
 - [Virtex App](https://github.com/34-Engineering/Virtex-App): Angular + Electron desktop application used for updating, viewing the camera stream of, and configuring Virtex.

## Images
![](/static/images/virtex2_on.jpeg)

![](/static/images/virtex2_desk.jpeg)

### PCB

This was an actually rediculous 8-layer PCB cramming
a USB chip, FPGA, and image sensor onto one tiny board.

![](/static/images/virtex_main_pcb_back.jpeg)

![](/static/images/virtex_main_pcb_front.jpeg)

