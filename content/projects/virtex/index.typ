#metadata((
  title: "virtex",
  desc: "815fps FPGA-based IR machine vision camera",
  started: "2021-04-30",
  ended: "2022-05-28",
  lang: "SystemVerilog",
  links: (
    ("HDL", "https://github.com/34-Engineering/Virtex-HDL"),
    ("PCBs", "https://github.com/34-Engineering/Virtex-PCBs"),
    ("CAD", "https://github.com/34-Engineering/Virtex-CAD"),
    ("App", "https://github.com/34-Engineering/Virtex-App")
  ),
  homepage: false
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post

= Context
In FRC (#link-new-tab("FIRST Robotics Competition", "https://www.firstinspires.org/programs/frc/")), its quite common to need to use a camera to track the position of target.
For a long time, these targets were simply retro-reflective tape.
Teams struggled with this challenge for a long time, but eventually the #link-new-tab("LimeLight", "https://limelightvision.io/") came out, which enabled teams to easily setup vision systems.
It was a camera module, RPi compute module, custom PCB, green LEDs, and fan packaged into a nice module, with pre-made software and FRC integration.

The LimeLight made FRC vision systems easy, buts it's high latency (50-100ms), required a lot of workarounds.
If it were placed on a turret, which was then on a moving robot, the control algorithms simply could not run off of its data directly.
Many teams, including mine, would run control algorithms on encoder positions, updating their target position based on LimeLight data.
Additionally, latency compensation was often necessary -- updating the target encoder position based off where it was when the image was taken, not where it is now.

Furthermore, LimeLight's use of green LEDs to highlight the retro-reflective target, led to unrealibility in different environments.
It was almost necessary for teams to tune their LimeLight vision pipelines at each competition, and if sunlight was involved, it could break the entire system. 

= Introduction
I wanted to make something better than LimeLight, and started this project.
I had two keys ideas:
 + Use 940nm (barely emitted by the sun) to avoid the green LED problem
 + Do all the vision processing on an FPGA for "real-time" processing

This project was extremely difficult, requiring three of the most complicated PCBs I had designed before, a custom housing, working with Chinese manufacturers, and a lot of SystemVerilog.

While the project was successful, FRC switched from retro-reflective tape to April tags before I released it -- something that is _probably_ impossible to fully process on a reasonably priced FPGA.

= Implementation

I wanted the camera to be quite small, which made the PCB design exceptionally hard.
The final version had 3x 1.5" x 2" PCBs: an aluminum PCB for the 940nm IR LEDs, a PCB for power and connections, and an eight-layer PCB.
The eight-layer PCB held most of the complexity: an Artix-7 FPGA (XC7A35T), FTDI USB chip, and Onsemi Python 300 image sensor (915 fps \@ 480p).

To keep the latency down, I opted for single-pass vision processing with no external RAM (extremely uncommon).
I couldn't find anything about this style of vision processing (single-pass, no RAM, quadrilateral detection), and had to come up with something completely novel.
I certainly don't think it's the best design, but it did work.
Initially, I prototyped designs in MATLAB, and later in TypeScript.
While somewhat helpful, doing this also hurt me, because I'd try to map traditional code into HDL, which didn't make sense.

(Maybe one-day I'll compile my massive Virtex log into this post)


= Images
#html.elem("img", attrs: (
  src: "virtex/virtex_main_pcb_back.jpeg",
  width: "50%"
))

#html.elem("img", attrs: (
  src: "virtex/virtex_main_pcb_front.jpeg",
  width: "50%"
))

#html.img(src: "virtex/virtex2_on.jpeg")

#html.img(src: "virtex/virtex2_desk.jpeg")
