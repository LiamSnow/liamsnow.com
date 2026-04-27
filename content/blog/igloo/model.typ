#metadata((
  title: "Modeling Smart Home Devices",
  desc: "How existing platforms model devices, and how Igloo will",
  written: "2025-08-27",
  updated: "2026-04-26",
  links: (
    ("Igloo", "/projects/igloo"),
  ),
  homepage: true
)) <page>

#import "../../_shared/template.typ": post, link-new-tab
#show: post


= Introduction
Automations and dashboards don't interact with vendor-specific interfaces for every device, but instead, a model the platform provides.
This model determines how they can control devices and read their current state.

I'm going to grade them on two scales:
 - *Cohesion*:
   - Allowing two lights from different vendors to be controlled together
   - Allowing an RGBCT and normal light bulb to be turned off together
 - *Coverage*:
   - How many device features are exposed
   - Exposing nonstandard features like reboot buttons, diagnostic sensors, light effects, etc.
   - Allowing new/unplanned devices to be modeled   

= Existing Solutions
Interestingly, every popular platform's model uses composition in some regard.
Devices are not simply modeled as a light or air conditioner, but composed of smaller units.
How fine-grained the units are, how they are put together, and other implementation details vary drastically.

== Home Assistant, Amazon Alexa, Google Home
#link-new-tab("Home Assistant", "https://developers.home-assistant.io/docs/core/entity/"),
#link-new-tab("Amazon Alexa", "https://developer.amazon.com/en-US/docs/alexa/device-apis/smart-home-general-apis.html"), and
#link-new-tab("Google Home", "https://developers.home.google.com/cloud-to-cloud/traits")
have surprisingly similar models, composing devices in a flat structure of strictly defined units.
This approach is relatively simple and provides strong cohesion for devices which fit into the definitions.

These definitions vary between platforms, but all hold a lot of information, and must balance coverage and bloat.
With too much coverage, definitions become massive, and hold features which many devices don't have.

When a device's feature cannot fit into an existing definitions, all provide three generic units: mode, number, and toggle.
The mode unit contains all potential modes (a list of strings) and the current mode.
Two vendors might represent the same functionality, with slightly different names, breaking cohesion.

== Apple HomeKit
#link-new-tab("HomeKit", "https://developer.apple.com/documentation/homekit/characteristic-types")
introduces a hierarchy, where devices are composed services, which are further composed of fine-grained characteristics (`Brightness`, `Volume`, `TargetTemperature`).
With the plethora of characteristics they define, many devices can be represented.
This model achieves strong cohesion between all devices, regardless of if the model was explicitly designed for them.

== Samsung SmartThings & Matter
Similarly,
#link-new-tab("SmartThings", "https://developer.smartthings.com/docs/devices/device-profiles") and
#link-new-tab("Matter", "https://docs.silabs.com/matter/latest/matter-fundamentals-data-model/")
have a hierarchical model, but with significantly more flexible structures.
This allows for nearly any device to be represented, but have zero cohesion in of themselves.
Instead, they define standards for vendors to follow when implementing their devices (#link-new-tab("1", "https://developer.smartthings.com/docs/devices/capabilities/proposed") #link-new-tab("2", "https://csa-iot.org/wp-content/uploads/2023/10/Matter-1.2-Device-Library-Specification.pdf")).

== Comparison Table

#[
  #import html: *

  #let g(x) = td(class: "good")[#x]
  #let o(x) = td(class: "okay")[#x]
  #let m(x) = td(class: "meh")[#x]
  #let b(x) = td(class: "bad")[#x]

  #table()[
    #tr()[
      #th[]
      #th[*Home \ Assistant*]
      #th[*Amazon \ Alexa*]
      #th[*Google \ Home*]
      #th[*Smart \ Things*]
      #th[*Apple \ HomeKit*]
      #th[*Matter*]
    ]

    #tr()[
      #td[*Levels of Hierarchy*]
      #td[1]
      #td[1]
      #td[1]
      #td[2]
      #td[2]
      #td[3+]
    ]

    #tr()[
      #td[#strong(title: "If device can contain multiples of the same unit, or a unit can contain multiples of the same sub-unit")[Duplicate Units]]
      #td[yes]
      #td[generics]
      #td[no]
      #td[no]
      #td[no]
      #td[no]
    ]

    #tr()[
      #td[*Cohesion \ Mechanism*]
      #td[41 strict unit definitions]
      #td[38 strict unit definitions]
      #td[38 strict unit definitions]
      #td[122 sub-unit standards]
      #td[135 strict unit defns.]
      #td[50 device standards]
    ]

    #tr()[
      #td[*Extended Coverage \ Mechanism*]
      #td[mode, number, toggle]
      #td[mode, number, toggle]
      #td[mode, number, toggle]
      #td[unstructured \ (no defns.)]
      #td[---]
      #td[mode]
    ]

    #tr()[
      #td[#strong(title: "Hierarchical models are represented as units + total sub-units")[Color Light]]
      #td[1]
      #td[4]
      #td[3]
      #td[1 + 4]
      #td[1 + 5]
      #td[1 + 3]
    ]

    #tr()[
      #td[#strong(title: "Hierarchical models are represented as units + total sub-units")[Fan with Light]]
      #td[2]
      #td[\~6]
      #td[\~5]
      #td[2 + \~7]
      #td[2 + \~8]
      #td[2 + \~6]
    ]

    #tr()[
      #td[#strong(title: "Cohesion for devices which fit into the models defined structure.")[Cohesion (defined)]]
      #o[7]
      #g[9]
      #g[9]
      #m[5]
      #g[9]
      #g[9]
    ]

    #tr()[
      #td[#strong(title: "Cohesion for devices which do not fit into the models defined structure.")[Cohesion (extended)]]
      #b[1]
      #m[4]
      #m[4]
      #b[0]
      #td[---]
      #b[3]
    ]

    #tr()[
      #td[*Coverage*]
      #b[3]
      #b[3]
      #b[3]
      #g[9]
      #o[6]
      #g[8]
    ]
  ]
]


== Conclusion

Typing styles:
 + Untyped / Flexible
   - Suffers from cohesion → requires standards
 + Typed with large units (ex. Home Assistant's `Light`)
   - Opinionated definition must balance coverage & bloat
   - Struggle with coverage → generic units (reduce cohesion)
 + Typed with small units (ex. `Dimmable`)
   - Cannot represent an N-channel relay without additional features
   - Allowing duplicate `Switch` units → can't distinguish/identify them
   - Hierarchy has each channel as unit composed of `Switch` and `Channel` → works well
   - Additional hierarchy → too flexible → requires standards

A strictly typed model with small units & one layer of hierarchy seems to be the best balance of cohesion and coverage.




= Igloo's Solution

The model we concluded with is freakishly similar to the  
#link-new-tab("ECS (Entity-Component-System)", "https://en.wikipedia.org/wiki/Entity_component_system")
model!
While it's really only used for games, it's a great reference to have, and we'll at least be using their naming scheme:
 + *Entity*: a set of components (no duplicates) only
 + *Component*: a primitive, strictly-typed data property
   - Contains a name like `Dimmer`
   - Optionally contains a value `Dimmer(f64)`
   - Otherwise, acts as a tag

An RGBCT light bulb becomes:
```rust
(Switch(true), Dimmer(1.0), Color(255, 0, 0),
  ColorTemperature(2000), ColorMode(Temperature), Light),
(Seconds(54), Uptime, Sensor),
(Decibels(−57), WiFiSignal, Sensor),
(Reset, Trigger),
...
```

=== Never Bloated, Backward-Compatible
If no existing components can model a feature, a new component is added, instead of modifying old ones.
Bloating the model is never a concern, as adding a new `FancyLightComponent` never affects existing lights, and is only attached to entities which have that behavior.
This append-only practice can easily be made backwards-compatible with automations or other code.


=== Device-Entity Relation
We have two primary options:
 + Actual Hierarchy:
   - Device entity exits, which relates to each entity which represents it
   - This does exist in other ECS implementations, like #link-new-tab("Bevy", "https://bevy-cheatbook.github.io/fundamentals/hierarchy.html")
 + Flat Tagging:
   - Give each entity something like a `Device(id)` component


=== Systems
\*I don't want to force Igloo into a corner here -- it may choose to have systems like ECS, or it may only have the EC part.

ECS separates data (entities and components) from logic (systems).
These systems are composed together and non-hierarchical.
They operate entirely independently, except for their interaction with entities and components.

#link-new-tab("Queries", "https://bevy-cheatbook.github.io/programming/queries.html")
are the method in which systems specify what entities/components they want to read, and what they want to change.
They define the shape of entity they are looking for, like ones with a `Dimmer` and `Switch`.
Queries are intentionally broad, and may find multiple matching entities.

When specificity is needed, tags (components with no values) can be used.
Tags can mark an entity as a `Light`, it's physical location like `Kitchen`, or anything else.
Notably, these tags give the user the option to use them, but doesn't require them to.


= Conclusion

I landed on the ECS-like device model before fully exploring what was out there.
I've questioned if this choice was right for a long time, and struggled to find anything else better.
But, going through and really understanding _what_ makes a model good (or bad) was worthwhile.
Now, I can confidently say this is the right model.
