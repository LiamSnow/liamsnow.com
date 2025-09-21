---
title: opensleep 81★
desc: Open source firmware for the Eight Sleep Pod 3
date: 2025-09-04
homepage: true
highlight: true
---

[GitHub Repo](https://github.com/liamsnow/opensleep)

# Table of Contents
<!--toc:start-->
- [Table of Contents](#table-of-contents)
- [Disclaimer](#disclaimer)
- [Quick Background](#quick-background)
- [Version 1](#version-1)
- [Version 2](#version-2)
  - [Features](#features)
  - [Difficulties](#difficulties)
    - [Deserializing](#deserializing)
    - [Baud Switching](#baud-switching)
    - [Reliability](#reliability)
  - [Random](#random)
  - [Publishing](#publishing)
    - [Mistakes](#mistakes)
- [Outcome](#outcome)
<!--toc:end-->

# Disclaimer
This project is purely intended educational and research purposes. It is for personal, non-commercial use only.
It is not affiliated with, endorsed by, or sponsored by Eight Sleep.
The Eight Sleep name and Pod are trademarks of Eight Sleep, Inc.

# Quick Background
The Eight Sleep Pod 3 is a temperature controlled (using water) mattress cover with sleep tracking.

# Version 1
I got a used Eight Sleep and used it for a while, but eventually wanted to have more control over it.

I heard about the [Nine Sleep Project](https://github.com/bobobo1618/ninesleep) and was super interested in it.
I got it setup and running but was a little frustrated with manually controlling it.
Furthermore, I really wanted to have all the features that original firmware had like
automatically setting alarms, temperature profiles, heat alarms, etc. 

I made a fork of the repo, reorganized the code and implemented all those features.
I've been super happy with it and have been using it for about a year.

Basically, it pretends to be the Device-API-Client (DAC - the command interface to the Eight Sleep servers)
and sends commands to set the bed temperature, alarms, etc.
It's really not too complicated, most of the complexity is in keeping reliable communications and sleep
scheduling.

You can check it out [here](https://github.com/LiamSnow/opensleep/tree/89ec7f39edceb2ad016dabdfdc469139db87eea7).

# Version 2
Version 2 was a TON (over ~6 months) of work to get right. It's a lot to explain exactly the difference
(I would check out the [README](https://github.com/LiamSnow/opensleep/blob/main/README.md))
but basically instead of pretending to be one of the Eight Sleep services and interacting
with the others, it just replaces ALL of the Eight Sleep services:

![opensleep comparison diagram](https://raw.githubusercontent.com/LiamSnow/opensleep/refs/heads/main/images/main.svg)

(Note that in this diagram ninesleep is the same as opensleep V1)

## Features

With opensleep V2 you can use your Pod 3 with complete privacy and make cool Home Assistant automations for
when you get in and out of bed. Personally I have it set up to read my daily calendar when I get out of
bed in the morning and remind to go to bed when its late. 

1.  **MQTT** interface for remotely updating config and monitoring state
2.  Configuration via **[Ron](https://github.com/ron-rs/ron)** file
3.  Presence detection
4.  Custom temperature profile with as many points as you want. It will spready out this profile between `sleep` and `wake` time.
5.  Vibration alarms relative to `wake` time (offsets and vibration settings can be configured)
6.  `Solo` or `Couples` modes
7.  LED control & cool effects
8.  Daily priming

## Difficulties
_I'm going to do my best to explain this assuming you haven't read the background in the README._

My original version that pretends to be the DAC interfaces with frankenfirmware (a C++ binary). Frank
then has USART communication to two STM32s called Sensor (manages all bed sensors) and Frozen (manages
temperature control system and everything water related).

Completing Version 2 of opensleep, meant figured out what the custom USART protocol is for Sensor and Frozen.

I had a combination of different efforts at first, but eventually landed on making a harness for frankenfirmware that
would monitor system calls leveraging [lurk](https://github.com/JakWai01/lurk). In this hardness, which I called Fiona (edited
version of opensleep V1), I can send commands to Frank and see how Frank responds. After some time with figuring
out the details of the protocol and incrementally improving Fiona, I managed to figure out most of the protocol.

It follows this basic structure:
```
7E [Length] [Command] [Data...] [CRC-CCITT 0x1D0F Checksum (2 bytes)]
```

Most of the complexity lies in:
 - What are the commands and what do they respond with
 - Deserializing the responses
 - Baud switching (Sensor starts at a lower baud rate and switches to a higher baud once you tell it to)
 - **Making it reliable**

### Deserializing
In order to deserialize really fast and reliably, I landed on just using raw indexing.
I totally understand cringing a little when you hear that, but I managed to make it reliable
with extensive testing, and it is just really fast.

Here's an example:
```rust
fn parse_capacitance(buf: BytesMut) -> Result<Self, PacketError> {
    validate_packet_size("Sensor/Capacitance", &buf, 27)?;

    let indices_valid = buf[9] == 0
        && buf[12] == 1
        && buf[15] == 2
        && buf[18] == 3
        && buf[21] == 4
        && buf[24] == 5;

    if !indices_valid {
        return Err(invalid_structure(
            "Sensor/Capacitance",
            "invalid indices".to_string(),
            buf,
        ));
    }

    Ok(Self::Capacitance(CapacitanceData {
        sequence: u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]),
        values: [
            u16::from_be_bytes([buf[10], buf[11]]),
            u16::from_be_bytes([buf[13], buf[14]]),
            u16::from_be_bytes([buf[16], buf[17]]),
            u16::from_be_bytes([buf[19], buf[20]]),
            u16::from_be_bytes([buf[22], buf[23]]),
            u16::from_be_bytes([buf[25], buf[26]]),
        ],
    }))
}
```

### Baud Switching
I managed to make the baud switching super reliable, actually more so than Frank.

When Sensor boots up I run something that I called "discovery" where it tries
at the lower (bootloader) baud rate for a few pings, if that doesn't work it switches
to the higher (firmware) baud rate for some pings. If both fail then I know it's disconnected/broken.

The reason you would get in a situation with it already at a higher baud rate, is if Frank or opensleep
was running already, then stopped, and restarted. Frank cannot really recover from this problem but
opensleep handles it perfectly.

### Reliability
My first prototype would simply just read single bytes until it found a start byte,
then try and read a packet. Sometimes it would just get stuck in an infinite loop
of being stuck in the middle of packets (thinking its finding start bytes but actually not).

In trying to fix this system I actually improved other parts of the code to. I am now using
a `Framed<SerialStream, PacketCodec<P>>` which means you can simply call `port.next()` and just
get back packets!

This codec is pretty awesome, basically what it does is:
 1. Advance bytes until it finds a start bytes
 2. Reads the packet
 3. Checks the checksum
   - If Valid -> Consume bytes and parse packet 
   - If Invalid -> Skip only the start byte and try again

Basically, it won't get "stuck in the middle" because it will only skip the start byte
if the checksum was bad instead of the whole packet (or what it thought was a packet).

## Random
In this project I also made the first Rust controller for the IS31FL3194 LED controller
which was more difficult than I thought.

It was also the first complex MQTT project I had done which was really awesome. I think I
made a really solid system that's super easy to interface with Home Assistant.


## Publishing
After spending so much time perfecting this project, I realized at some point I just needed
to release it and get some feedback.

I made two Reddit posts, one on
[r/rust](https://www.reddit.com/r/rust/comments/1n8hu4p/opensleep_rust_firmware_for_the_eight_sleep_pod_3/)
and one on
[r/eightsleep](https://www.reddit.com/r/EightSleep/comments/1n8ppn8/opensleep_complete_firmware_for_the_eight_sleep/).

It was pretty successful! I got 373 upvotes in total and 81 stars on [GitHub](https://github.com/liamsnow/opensleep).
People seemed to really like the project which was a great feeling after spending so much time on it.

### Mistakes
It was my first time publishing a programming project on Reddit and really finishing something in GitHub and
I certainly made some mistakes, notably:
 - Went way to deep into technical detail in the post (should have kept it in GitHub)
 - I didn't clearly explain context: What is an Eight Sleep? Why would someone use this project? What is Frozen? ...

After some helpful comments I went through a ton of versions revising both of my posts and the
GitHub README. I think now its in a pretty understandable state, but it's a great lesson for me to learn.

# Outcome
I am super happy that I decided to do this project and publish it. It taught be a lot about how
to publish programming projects and how to implement binary protocols in Rust.
