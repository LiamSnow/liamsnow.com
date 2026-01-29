#import "../shared/template.typ": post
#show: post.with(
    base: "projects",
    routes: toml("routes.toml"),
    filename: "opensleep.typ"
)

#link("https://github.com/liamsnow/opensleep")[GitHub Repo]

= Disclaimer
This project is for educational and research purposes only. It is for personal, non-commercial use and is not affiliated with, endorsed by, or sponsored by Eight Sleep. The Eight Sleep name and Pod are trademarks of Eight Sleep, Inc.

= Background

The Eight Sleep Pod 3 is a smart mattress cover that uses water circulation to control temperature (ranging from 55°F to 110°F). Temperature regulation during sleep has been shown to improve sleep quality, increasing deep sleep by up to 14 minutes and REM sleep by up to 9 minutes. The Pod also tracks sleep metrics like heart rate, HRV, and sleep stages.

However, Eight Sleep's closed-source firmware has many privacy concerns.
By default, raw sleep tracking data is streamed directly to Eight Sleep's
servers. Eight Sleep can see when you're in bed, how many people are in bed,
your sleep patterns, and more.

I got a used Eight Sleep Pod and, after using it for a while, wanted more control.
I discovered #link("https://github.com/bobobo1618/ninesleep")[ninesleep] which provided a foundation, but I needed features like automatic alarm scheduling, temperature profiles, and heat alarms that the original firmware offered.

= Version 1

I forked Nine Sleep, reorganized the code, and implemented all the features I wanted.
Version 1 (and ninesleep) pretends to be the Device-API-Client (DAC).
The DAC is the interface between the Eight Sleep servers and the lower
level firmware.

I used this version happily for about a year. You can check it out #link("https://github.com/LiamSnow/opensleep/tree/89ec7f39edceb2ad016dabdfdc469139db87eea7")[here].

While this version works great, it doesn't allow access to the raw sleep tracking
data immediately. Instead, it only receives batched updates.

= Version 2

Version 2 took approximately six months to complete and represents a complete reimplementation. Instead of pretending to be one Eight Sleep service (the DAC) and interacting with the others, it replaces all Eight Sleep services entirely:

#image("opensleep.svg")

(Note that in this diagram ninesleep is equivalent to opensleep V1)

== Features

With opensleep V2, you can use your Pod 3 with complete privacy and create Home Assistant automations for bed presence. I have mine configured to read my daily calendar when I get out of bed in the morning and remind me to go to bed when it's late.

+ *MQTT* interface for remote configuration and state monitoring
+ Configuration via *#link("https://github.com/ron-rs/ron")[Ron]* file
+ Presence detection
+ Custom temperature profiles with unlimited points, automatically distributed between `sleep` and `wake` times
+ Vibration alarms relative to `wake` time (offsets and vibration settings are configurable)
+ `Solo` or `Couples` modes
+ LED control with custom effects
+ Daily priming

== Technical Challenges

Version 1 interfaces with frankenfirmware (a C++ binary), which communicates via USART to two STM32 microcontrollers: "Sensor" (manages bed sensors) and "Frozen" (manages temperature control and water systems).

Completing Version 2 meant reverse-engineering the custom USART protocol for Sensor and Frozen. I tried several approaches before building a harness for frankenfirmware using #link("https://github.com/JakWai01/lurk")[lurk], a modern Rust alternative to strace for monitoring system calls. This harness, which I called Fiona (a modified version of opensleep V1), let me send commands to frankenfirmware and observe its responses. Over several months of incremental improvements, I decoded most of the protocol.

The protocol follows this structure:
```
7E [Length] [Command] [Data...] [CRC-CCITT 0x1D0F Checksum (2 bytes)]
```

The main complexity involved:
- Finding out the protocol structure
- Determining available commands and their responses
- Deserializing responses accurately
- Handling baud rate switching (Sensor starts at a lower baud rate and switches after initialization)
- Making communication reliable

=== Baud Switching

Baud switching is more reliable than frankenfirmware's implementation. When Sensor boots, I run a "discovery" process: try pings at the lower (bootloader) baud rate, then switch to the higher (firmware) baud rate if that fails. Both failing indicates disconnection.

This handles the case where frankenfirmware or opensleep was already running, stopped, and restarted in a short amount of time. Frankenfirmware cannot recover from this, because it only tries at the bootloader baud. Opensleep handles it seamlessly.

=== Reliability

My first prototype read bytes sequentially until finding a start byte, then attempted to parse a packet. It would sometimes get stuck in infinite loops, thinking it found start bytes in the middle of packets.

The solution uses a `Framed<SerialStream, PacketCodec<P>>` which provides packets directly via `port.next()`. The codec:
+ Advances bytes until finding a start byte
+ Reads the packet
+ Checks the checksum
   - If valid → consume bytes and parse packet
   - If invalid → skip only the start byte and retry

This prevents getting stuck in the middle of packets because it only skips the start byte on checksum failures, not the entire (potentially incorrect) packet.

== Additional Components

I also implemented the first Rust controller for the IS31FL3194 LED controller, which was more challenging than anticipated.
I couldn't actually find the LED controller on the PCB, but after a lot of digging
I was able to guess based off the I2C commands sent during boot.
The IS31FL3194 is a 3-channel LED driver with two-dimensional auto breathing.
Basically, the LED patterns live on-device and are configuring using I2C instead
of on the main processor.
I implemented the bulk of the features, which allows for some cool lighting
effects like an RGB breath.

This was my first complex MQTT project. I built a system that integrates cleanly with Home Assistant.

== Release & Community Response

After months of development, I decided to release the project and get feedback. I posted on #link("https://www.reddit.com/r/rust/comments/1n8hu4p/opensleep_rust_firmware_for_the_eight_sleep_pod_3/")[r/rust] and #link("https://www.reddit.com/r/EightSleep/comments/1n8ppn8/opensleep_complete_firmware_for_the_eight_sleep/")[r/eightsleep].

The response was encouraging: 373 combined upvotes and 112 GitHub stars. People are actively using it, which validated the effort.

=== Lessons Learned

This was my first time publishing a significant project on Reddit and GitHub. I made several mistakes:

- Went too deep into technical details in Reddit posts (should have kept that in GitHub)
- Didn't clearly explain context: What is Eight Sleep? Why would someone use this? What is Frozen?

After helpful feedback, I revised both posts and the GitHub README multiple times. The documentation is now much clearer, but it taught me the importance of assuming no prior knowledge when introducing projects.

= Outcome

This project taught me how to reverse-engineer binary protocols in Rust and how to publish open source projects effectively. The fact that others are using opensleep makes the six months of development worthwhile.
