#metadata((
  title: "opensleep",
  desc: "Open source firmware for the Eight Sleep Pod 3",
  started: "2024-11-05",
  ended: "2025-12-01",
  lang: "Rust",
  links: (
    ("GitHub", "https://github.com/liamsnow/opensleep"),
  ),
  homepage: true,
  highlight: true
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post

= Disclaimer
This project is for educational and research purposes only. It is for personal, non-commercial use and is not affiliated with, endorsed by, or sponsored by Eight Sleep. The Eight Sleep name and Pod are trademarks of Eight Sleep, Inc.

= Background

The #link-new-tab("Eight Sleep Pod 3", "https://www.eightsleep.com/blog/announcing-pod-3/")
is a smart mattress cover that uses water circulation to control temperature (ranging from 55°F to 110°F). The Pod also tracks sleep metrics like heart rate, HRV, and sleep stages.

I was frustrated that many features of the bed were paywalled behind a monthly subscription. Furthermore, I was concerned that it streams raw sleep-tracking data to Eight Sleep's servers at all times. The
#link-new-tab("CEO makes it very clear", "https://x.com/m_franceschetti/status/1726732560770666979")
that he has access to everyone's sleep tracking data. Everyone at Eight Sleep knows when you’re in bed, how many people are in bed, your sleep patterns, and more. They also have an
#link-new-tab("SSH backdoor into your bed", "https://trufflesecurity.com/blog/removing-jeff-bezos-from-my-bed").
Sadly, there’s no simple solution to these problems, as blocking it from the internet breaks the mobile app. 

= Version 1

After some digging, I found articles about people hacking into their Eight Sleep pods. Basically, you take apart your Pod, edit the backup Yocto Linux install to include your WiFi and SSH keys, and trigger a factory reset. Once inside, you can see that the mobile app communicates to a TypeScript project called the Device-API-Client (DAC), which then sends commands to the C++ firmware, Frank, over a Unix Socket. One project,
#link-new-tab("ninesleep", "https://github.com/bobobo1618/ninesleep"),
replaces the DAC with a Rust program that exposes a local “raw” REST API to communicate with the firmware.

While ninesleep is a great project, I wanted automatic control of the bed, as you would get from the mobile app. I created a
#link-new-tab("fork", "https://github.com/LiamSnow/opensleep/tree/89ec7f39edceb2ad016dabdfdc469139db87eea7"),
cleaned up the code, and added many features, such as alarms and temperature profiles. I was very happy with it and used it for about a year.

I briefly explored getting sensor data by editing `/etc/hosts` to replace `raw-api-upload.8slp.net:1337` with my own server. Then Frank, the firmware, would send batches of raw sensor data.

= Version 2

For a long time, I wanted to set up automations around the sensors on the bed. For example, reading my calendar and turning off my alarm when I got out of bed in the morning. Sadly, the firmware will only send batches of sensor data, meaning it wouldn’t work for the automations I wanted.

Getting sensor data immediately required completely replacing Frank. This is what Version 2 is, a replacement for all of Eight Sleep's programs running on the pod. It was a lot more work than V1, required reverse-engineering a C++ binary, bespoke UART protocols, and more.

#html.img(src: "opensleep/comparison.svg")

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

= V2 Story
I took apart my Eight Sleep again, making diagrams and
#link-new-tab("notes", "https://github.com/LiamSnow/opensleep/blob/main/BACKGROUND.md")
to figure out the hardware.
Most importantly, I learned that Frank communicates with two STM32 microcontrollers over UART:
 - "Frozen" to control everything water (pumping, priming, heating, and cooling)
 - "Sensor" to collect data from all sensors (capacitance, piezo, temperature).

I now knew that V2 would have to be able to communicate with these MCUs.
Using strace, I both learned the device paths for the UARTs (`/dev/ttymxc0`, `/dev/ttymxc2`) and the basic protocol structure:

```
7E [Length] [Command] [Payload...] [CRC-CCITT 0x1D0F Checksum (2 bytes)]
```

== Fiona
Knowing the command structure allowed me to create a more formal tool, Fiona:
 - Runs Frank & traces its system calls
   - Prints logs from it
   - Tracks what location Sensor and Frozen are opened on, then uses this to track reads & writes
 - Uses parts of V1 to communicate with Frank over the Unix Socket

Using this, we can understand a lot:
 + Initialization sequence
 + Idle behavior (ex. pinging)
 + Frank's response to Unix Socket commands
   - Can see immediate response
   - Also can see response over time (ex. requesting a temperature for a duration can lead to multiple calls over time)

I iteratively built out Fiona, applying what I learned about the protocol back into itself. For example, if I learned a new command, I would add it into Fiona.

=== Sensor Protocol Modes
A key finding was that the Sensor protocol has two modes:
 + *Bootloader* (starts here): 38,400 baud
   - Used for configuration + getting hardware information
   - (Unconfirmed) Used to flash new firmware
   - `0x10` command will request the MCU to jump to Firmware mode
 + *Firmware*: 115,200 baud
   - Higher baud required to handle amount of data being sent (piezo sensor sends a ton of data)
   - Capacitance sensor data always sent (2Hz)
   - Bed temperature send on request ("probing")
   - Piezoelectric sensors sent after being configured (gain, frequency, ...) and enabled 

== MVP Implementations
At some point, I hit a wall with Fiona and just had to start implementing the protocols. Fiona taught me a lot, but there was still a lot more nuance to learn about and (more importantly) how to deserialize the payloads.

== Start Byte Problem
Naively, I built a system that would scan for the start byte (`0x7E`) and then read packets.
This is flawed and bad practice in general.
A `0x7E` might exist inside of payload, and not actually be a start byte. 
If the program encounters this, it falls apart:
 + Reads invalid start byte
 + Reads invalid length byte
 + Reads `length` bytes
   - Commonly reads to the end of this packet and into the next, which misses the next actual start byte

 I quickly replaced this with a
#link-new-tab("Tokio codec", "https://github.com/LiamSnow/opensleep/blob/main/src/common/codec.rs"), which:
+ Advances until a start byte
+ Reads the packet
+ Checks the checksum
   - If valid → consume bytes and parse packet
   - If invalid → skip only the start byte and retry

This completely resolves the problem. Initially, it fails to deserialize packets, but eventually finds a valid packet, and operates properly from there.

Additionally, it provides great ergonomics. After wrapping the serial port
(`Framed<SerialStream, PacketCodec<P>>`)
you can simply call `port.next()` and receive packets.


== Configuration in Sensor Protocol
The sensor protocol has configuration in both modes (Bootloader & Firmware).
It is very common for configuration commands to fail, which means they
need to be retried. Knowing when to retry a configuration requires:
 + Knowing response packet ID (which I learned is always `cmd_id + 0x80`)
 + Serialization of command, deserialization of response, and their correlation
   - You need to know what you're requesting and then if the response was "good" or "bad"
   - For piezo gain, the command payload 2x `u16`'s (left and right gains) and the response payload is identical. However, the gain cannot be perfectly matched -- requesting `400` yields `405` in my case. Other commands have similar behavior, and this must be accounted for. 


== Sensor Protocol Scheduling

While Frozen is pretty simple and has no complex scheduling requirements, Sensor is quite the opposite. It requires:
 + Configuration command retries
 + Command precedence (piezo should be fully configured before enabling)
 + Multiple interval-based commands (pinging + temperature probing)
 + Proper spacing betweeen commands (a spam of commands can lead to dropped commands or breaking the protocol)

Knowing this, I chose a tick-based (20hz) scheduler system because its reliable and intuitive. It takes in a set of pre-defined commands, each with an interval, run condition, and offset:
 - Staggers interval based commands (pinging & probing have the same period & are offset 90 degrees from each other)
 - Configuration commands have a 800ms interval and run conditions:
   - Prevents retry spam
   - Only runs if the configuration is not set
   - Enforces precedence and other requirements


== Firmware Mode Locking
After the MCU is requested to enter Firmware mode, it remains here for a long period of time (>10 minutes IIRC). This is not a problem that Frank cares about (because it doesn't get restarted often), but its something that makes development of the protocol quite annoying.

I developed a "discovery" system to solve this:
 + Tries to connect in Bootloader mode. If it works, request to jump to Firmware mode, and wait for it to do so
 + If Bootloader mode fails, try to directly connect in Firmware mode

After this, simply re-apply all configuration. This works because of two key details:
 + There is no way to request a downgrade from Firmware → Bootloader that I could find
 + All configuration _can_ occur in Firmware mode


== I2C Enabling
When developing, I disabled all Eight Sleep processes. After a reboot, I noticed that the MCUs didn't work at all.
After diagnosing the problem, I narrowed it down to Capybara, their C\# "supervisor" and Bluetooth setup program.

Eventually, I found I2C commands in boot scripts and while tracing Capybara.
These commands control an I2C multiplexer that both enables the microcontrollers and controls the front LED. This was a simple addition to the crates and solved the problem.

== LED Controller
After doing I2C enabling, I decided to get the LED working.
I tore apart my Eight Sleep again, trying to find the LED controller, but failed.

I spent a long time researching I2C LED controllers and couldn't find anything that matches the I2C commands.
Eventually I realized the core feature -- it was a "breathing" LED controller.
Knowing this, I managed to identified it as the
#link-new-tab("IS31FL3194", "https://www.lumissil.com/assets/pdf/core/IS31FL3194_DS.pdf").

I worked on implementing the
#link-new-tab("first Rust controller", "https://github.com/LiamSnow/opensleep/tree/f2d1c7f82ad746b33c2437a667ce4c69ac93cd96/src/led")
for it.
The documentation is pretty horrible, and there was only basic Arduino libraries I could reference. It really brought me back to my PCB designing days.

The interface is complex because the entire LED effect is shipped out to the controller. I struggled with architecting a good interface for it, but eventually landed on a pretty good system, allowing for various effects like RGB scrolling.

== Putting it Together
After months of reverse-engineering, I felt happy with my implementation of the protocols and brought them into the main repository.
I de-duplicating as much of their logic as I could.
Then, I built a new temperature-profile system, a configuration system, presence detection, and more. I centered it around MQTT, simplifying the interface and enabling opensleep to push to subscribers.

== Publishing
I posted opensleep on #link("https://www.reddit.com/r/rust/comments/1n8hu4p/opensleep_rust_firmware_for_the_eight_sleep_pod_3/")[r/rust] and #link("https://www.reddit.com/r/EightSleep/comments/1n8ppn8/opensleep_complete_firmware_for_the_eight_sleep/")[r/eightsleep].
This was my first time publishing a project, and I made several mistakes:
- Went too deep into technical details on r/eightsleep, which confused people (many were non-technical)
- Didn't explain what the Eight Sleep was on r/rust, so people had no idea what I was talking about
- Made exceptionally long Reddit posts, which discouraged readers
- Tried to both ask for feedback and publish the project on r/rust, which didn't make much sense
- Tried to combine the story and explanation of the project in the README, which made it long and confusing

After helpful feedback, I revised both posts and the README.
Even so, the project landed well, receiving 373 total upvotes and 126 GitHub stars.

= Outcome
I never thought that my initial purchase of a broken Eight Sleep Pod 3 would lead me here, but I am so happy that it did.
I learned a lot about reverse-engineering, writing protocols in Rust, making a good MQTT interface, and how to publish a project.
But most importantly, I created something that people actually use daily.
