#metadata((
  title: "virtex",
  desc: "815fps FPGA-based IR machine vision camera",
  started: "2021-04-30",
  ended: "2022-05-28",
  lang: "SystemVerilog",
  links: (
    ("GitHub", "https://github.com/34-Engineering"),
  ),
  homepage: false
)) <page>

#import "/_shared/template.typ": post
#show: post

During COVID-19, I designed an FPGA-based vision camera for FIRST Robotics Competition to address latency issues in existing solutions like LimeLight. LimeLight had 50-100ms latency, which required control algorithms to compensate for outdated position data. For example, a turret tracking a target needs to account for where the target was 50ms ago, not where it is now.

Virtex achieved 815fps retro-reflective target tracking with minimal latency, eliminating the need for latency compensation in control systems.

== Technical Approach

The main challenge was implementing vision processing entirely in the FPGA pipeline without external RAM. Traditional vision processing uses multiple stages with intermediate buffering, but I had to design a single-pass pipeline that processed each frame in one shot. This constraint required reimagining standard vision algorithms to work within the FPGA's block RAM limitations.

The hardware consisted of a compact 1.5" x 2" eight-layer PCB integrating an Artix-7 FPGA (XC7A35T), FTDI USB chip, and Onsemi image sensor.

== Outcome

I successfully demonstrated 815fps tracking, but FIRST Robotics Competition switched from retro-reflective tape to AprilTags before the project could be deployed. This architectural change made the specialized hardware obsolete, and I shelved the project.

= Sources
 - #link("https://github.com/34-Engineering/Virtex-HDL")[Virtex HDL]: Xilinx Vivado project for the Artix-7 FPGA
 - #link("https://github.com/34-Engineering/Virtex-PCBs")[Virtex PCBs]: Altium source for PCBs and adapter board
 - #link("https://github.com/34-Engineering/Virtex-CAD")[Virtex CAD]: Fusion 360 CAD files
 - #link("https://github.com/34-Engineering/Virtex-App")[Virtex App]: Angular + Electron desktop application for configuration and camera stream viewing

= Images
#html.img(src: "virtex/virtex2_on.jpeg")

#html.img(src: "virtex/virtex2_desk.jpeg")

== PCB

Eight-layer PCB integrating FTDI USB chip, Artix-7 FPGA, and Onsemi image sensor:

#html.img(src: "virtex/virtex_main_pcb_back.jpeg")

#html.img(src: "virtex/virtex_main_pcb_front.jpeg")
