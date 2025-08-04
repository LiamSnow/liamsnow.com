---
title: opensleep
desc: Open source firmware for the Eight Sleep Pod 3
date: 2025-06-10
---

[GitHub Repo](https://github.com/liamsnow/opensleep)

## How it Started
My Open Sleep project has been quite a journey.
Originally I had heard about the [Nine Sleep Project](https://github.com/bobobo1618/ninesleep)
and was super interested about it.
I got it setup and running but was a little frustrated with manually controlling it.
Furthermore, I really wanted to have all the features that original firmware had like
automatically setting alarms, temperature profiles, heat alarms, etc. 

I made a fork of the repo, reorganized the code and implemented all those features.
I've been super happy with it and have been using it for about a year.

## Technical Overview of the Eight Sleep Pod 3
Here is some context for the rest of the article. Note that this is a combination of work I have done and also gathered by the other great people working on ninesleep and freesleep.

### Main Controller
A Linux SOM (`VAR-SOM-MX8M-MINI_V1.x`) running pretty minimal Yocto build (no package manager, build tools, etc).
 - Systems runs off 8GB eMMC normally
 - Micro SD card (16GB industrial SanDisk) contains 3 partitions (p1 to boot from, p3 for persistent storage)
    - If the small button is held in during boot, the SOM will boot from the SD card p1
    - It will run a script that will copy `/opt/images/Yocto/rootfs.tar.gz` onto the eMMC, then reboots from eMMC

I have managed to install `Opkg` on it, so now I can proudly say I have neovim on my bed :) 

### Services
Frank (`/opt/eight/bin/frakenfirmware`) C++ binary with simple UNIX socket commands. Controls:
 - Sensor Unit (STM32F030CCT6) over UART (`/dev/ttymxc0`), flashes `firmware-sensor.bbin`
    - 6 capacitance sensors, 1x/second
    - 2 Piezo sensors, 500x/second
    - Bed temp (microcontroller's temp, ambient temp, humidity, 6 on bed)
    - Freezer temp (ambient, hs, left/right)
    - Vibration alarms
    - Takes in a left and right ADC gain parameter (default `400`)
 - "Frozen" over UART (`/dev/ttymxc2`), flashes `firmware-frozen.bbin`
    - Takes l/r temperatures and durations
    - Also handles priming
 - Uploading Raw sensor data + logs to `raw-api-upload.8slp.net:1337`

Capybara (`/opt/eight/bin/Eight.Capybara`) .NET code. Didn't look into this much but it seems to handle initial setup via bluetooth
 - Writes `/deviceinfo`
 - Has a loopback with the sensor UART (for debugging?)
 - Restarts Frozen when it gets stuck, seems like a consistent problem
 - LEDs over I2C (IS31FL3194)
   - Potentially also controlled by Frank?

Device-API-Client (DAC)/PizzaRat (`/home/dac/app`) Node TypeScript code
 - CoAP for device API `device-api.8slp.net:5684`
 - Basically just a wrapper for Frank, mobile app communication goes through this.

SWUpdate gets software updates from `update-api.8slp.net:443`

## Adding Sleep Tracking Data to Open Sleep
I spent quite a while referencing ninesleep which somewhat implements grabbing
sleep tracking data from Frank directly, but it was flawed and took a lot of working
for it to be consistent.

While working on that I actually discovered a new project in the area which
is pretty similar to what I have been looking for called
[Free Sleep](https://github.com/throwaway31265/free-sleep).
The reverse-engineering work they did was really useful while I was working
on my new version. A big note however, is they don't actually pretend
to be `raw-api-upload.8slp.net`, instead they just grab sleep tracking
data from the file in persistent storage, which has some downsides.

Something just never sat right with me. Through all my reverse engineering
work, I realized that Frank isn't actually doing _that_ much. Most of the heavy
lifting seems to be from the two microcontrollers (frozen and sensor) and
in the cloud for processing sleep tracking data. Furthermore, if I was able
bypass Frank, I could immediately access sleep tracking data. This
would allow for a lot of cool things in a smart home like having automations
trigger when you get into/out of bed.

## A New Version
I have spent countless hours sifting through `strace` in order to completely
reimplement the frank to frozen and frank to sensor protocols. Its not standard
at all and quite tediuos, but I have made great progress. I'm not quite
ready to release it yet, but I have a lot of information to dump here.

Sensor communicates over USART at `/dev/ttymxc0` and frozen at `/dev/ttymxc2`.

The protocol for both looks follows this structure:

```
7E [Length] [Command] [Data...] [Checksum]
```

The checksum is CRC-CCITT (0x1D0F).

There are some differences between the two but in general they have two modes:
 1. Bootloader mode. When you start communications at 38400 baud, it will jump
to this mode (in most cases). Here you can ping, get hardware information,
and tell it to go to firmware mode. Seems like a sleep mode basically.
 2. Firmware mode is where the actually work happens. To keep it in firmware mode
you have to keep pinging it.

### Frozen
You send a command which includes the bed side (left or right), target temperature
in celcius, and whether its enabled. Then it'll send back a target update which confirms this.

It will also periodically send state updates (left/right temps, heatsink temp, state).

### Sensor
This is where it gets a bit more complicated. Once exiting the bootloader phase,
the connection speed needs to be upgraded to 115200 baud so it can stream you all the data.

Theres a bunch of configuration you can give it like:
 1. Piezo gain left and right (normally just 400,400)
 2. Sampling rate (normally 1kHz)
 3. If it should sample the Piezo
 4. If it should enable vibration

It will always send you capacitance sensor readings. This happens ~every second
and it gives 6 readings, one for each sensor.

To get the bed temperature you have to ask it every single time. It will give you
back 4 temperature readings for each bed side, the ambient temperature, the humidity,
and the microcontroller temperature.

When enabled, the piezo samples will be sending really fast. Each one gives a frequency,
left/right gains, and the samples.



