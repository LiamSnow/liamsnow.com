#metadata((
  title: "Igloo",
  desc: "A truly smart, smart home platform",
  started: "2025-01-01",
  ended: "Now",
  lang: "Rust",
  homepage: true,
)) <page>

#metadata((blogs: "/blog/igloo/")) <query>

#import "../_shared/template.typ": post, link-new-tab 
#show: post

= Motivation

Most smart homes aren't smart.
They replace a light switch with a phone app or voice commands, physical timers with timer-based routines, and motion sensing lights with location-based automations.

In my opinion, a smart home should:
 + Be *semi-autonomous*: not always require initiation (through dashboard or voice)
 + Have *complex orchestrations* between many heterogeneous sensors and devices
 + Be *adaptable* and *dynamic*

Some theoretical ideas:
 - *Alerts* like calendar events, fire alarms, air quality problems, etc.
    - go beyond one-off notifications
    - display on screens & output sound
    - use multiple notification systems
    - escalate priority if unhandled
 - *Alarms*
    - control many devices like lights, speakers, bed temperature, AC, etc.
    - know off if you're in bed, and awake or deeply asleep
    - understand urgency like if today is an important event
 - *Initiate*: read your day plan when you get out of bed (instead of when prompted to)
    - must understand context: distinguish you getting out of bed in the morning from a nap or simply sitting in bed
    - adjust depending on: the day, if a guest is present, from calendar data, to-do lists changes, sleep data from that night, etc.

Existing graphical and markup-based (ex. YAML) smart home automation builders either make this impossible or impractical.
Instead, code must be used, but almost zero tooling exists to help.
Implementing the complex automations alone is hard, but the dynamic, adaptable, and interruptable parts are especially hard.
Practically, this is out of reach for a single person to develop and maintain for their home.



= Background

Smart devices have become cheaper, better, and abundant:
 - *Household*: lights, speakers, air conditioners, blinds, beds, locks, vacuums, washing machines, etc.
 - *Sensors*: energy meters, cameras, smoke detectors, air quality sensors, motion detectors, temperature sensors, pool pH sensors, etc.

How these devices communicate is equally varied.
Some professional systems use wired connections like Ethernet or proprietary serial buses.
Most consumer devices are wireless, but even among these, there is little consistency.
A Philips Hue bulb and a TP-Link plug may both be on the same Wi-Fi network but speak entirely different application-level protocols.
Others use low-power mesh networks like Zigbee, Z-Wave, or Thread, which require a dedicated hub or radio to bridge into the home network.

Smart home platforms bridge this, connecting together many devices.
But, the dominate commercial platforms (Alexa, Google Home, HomeKit, SmartThings) have vendor lock-in, forcing you to use their devices, or devices which support them.
They also have limited automation capability because they focus on appealing to nontechnical consumers.

#link-new-tab("Home Assistant", "https://www.home-assistant.io/") is much better alternative to these.
It's a FOSS platform you can throw on a Raspberry Pi which can basically work with any smart home devices (thousands of integrations).
It's automation system is also pretty limited, forcing Python for anything complex.


= This Project

Igloo is trying to enable the complex home automation that I want.
It's an automation system & a platform that empowers it.
Like Home Assistant, the platform will connect any smart home device, but it will additionally attempt to:
 - Expose more features from devices
 - Be extremely reliable
 - Have better performance
 - Consume less resources, enabling it run on more affordable hardware
 - Start from first principals and going against the status quo when justified

= Igloo's History

Igloo has been the biggest and longest project I have ever worked on.
I naively started this project thinking it would take month to make a simpler #link-new-tab("Home Assistant", "https://www.home-assistant.io/") replacement (#link("/blog/igloo/v1")[see V1 post]), but turns out, smart homes are trickier than they look!

And yet, I've stuck with this project for so long mostly because the programming is super interesting and it constantly challenges me.
It's improved me as programmer more than I could've imagined.

As time has gone on, I've changed many technical parts of the project, but also evolved the goal of it.
I kept trying to make a popular and intuitive Home Assistant competitor (#link-new-tab("see V2 branch", "https://github.com/LiamSnow/igloo/tree/v2")), but it forced the project into directions which I didn't want to go.

I wanted to "have my cake and eat it too:" an intuitive & accessible platform that could also enable really complex home automation.
These are usually contradictory, and at the end of the day, I realized I wanted the ladder (complex home automation).
I also just didn't want to be spending my time adding Python support and developing a dashboard with good UI/UX.





= Blog Posts

#let posts = {
  sys.inputs.at("blogs", default: ()).sorted(key: p => p.at("written", default: "")).rev()
}


#html.div(class: "posts")[
  #posts.map(post => html.a(href: post.url, class: "post")[
    #html.div(class: "title")[#post.title]
    #html.div(class: "desc")[#post.at("desc", default: "")]
  ]).join()
]


