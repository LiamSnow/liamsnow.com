#metadata((
  title: "esphomebridge-rs",
  desc: "ESPHome client API written in Rust",
  date: "2025-01-15",
  homepage: true
)) <page>

#import "/_shared/template.typ": post
#show: post

#link("https://github.com/liamsnow/esphomebridge-rs")[GitHub Repo]

*NOTE*: This is one of my earlier Rust projects and has since been rewritten for #link("https://github.com/LiamSnow/igloo/tree/main/esphome")[Igloo V2]. The new version is cleaner and faster, but I wanted to keep this post up.

= Background

#link("https://esphome.io/")[ESPHome] is customizable firmware for ESP microcontrollers, similar to Yocto but for smart home devices. You configure devices with YAML files, which generate C code that gets compiled and flashed to the microcontroller. Devices then communicate via a Protobuf-based protocol.

ESPHome devices were really only accessible through Home Assistant, which was created by the same team. The protocol was completely undocumented. The only existing library, #link("https://github.com/esphome/aioesphomeapi")[aioesphomeapi], is written in Python and designed specifically for Home Assistant core.

I needed ESPHome support for #link("https://liamsnow.com/igloo")[Igloo], which meant building the first implementation of the ESPHome protocol outside the Home Assistant ecosystem.

= Reverse Engineering

There was minimal prior work. #link("https://github.com/pixelspark")[pixelspark] had created #link("https://github.com/pixelspark/esphome-rs")[esphome-rs], but it had limited functionality. Fortunately, ESPHome uses Protobuf, which allowed me to automatically generate interface code.

The hard part was understanding the Noise protocol. Most new ESPHome devices (including all of mine) use Noise-encrypted connections by default. With no documentation available.

== Noise Protocol

Modern ESPHome devices use the Noise protocol for encrypted connections. I used the #link("https://crates.io/crates/snow")[snow] crate, configuring it with the parameters `Noise_NNpsk0_25519_ChaChaPoly_SHA256` and prologue `NoiseAPIInit\x00\x00`.

The handshake sequence:
+ Send hello (`0x01`)
+ Wait for hello response
+ Complete handshake

After the handshake completes, communication proceeds like a normal unencrypted connection.

== Protobuf Protocol

After connection setup, the Protobuf protocol begins. The client introduces itself with a name and supported versions, then makes a connection request (which includes a password for non-Noise connections).

Once connected, you fetch all entities and services. Services are scripts or voice assistant integrations on the device. Everything else is an entity (lights, switches, sensors, etc.).

= Implementation

The API focuses on simplicity and performance.

Connect to a device:
```rust
let dev = Device::new_noise("IP", "NOISE_PSK")?;
dev.connect().await?;
```

List all buttons:
```rust
for e in &dev.entities.button {
    println!("Button: {:#?}", e.1);
}
```

Turn on all lights:
```rust
for light in dev.entities.light {
    let req = api::LightCommandRequest {
        key: light.key,
        has_state: true,
        state: true,
        ..Default::default()
    };

    dev.light_command(req).await?;
}
```

Turn on a specific light by name:
```rust
let req = api::LightCommandRequest {
    key: dev.get_light_key_from_name("rgbct_bulb")?,
    has_state: true,
    state: true,
    ..Default::default()
};

dev.light_command(req).await?;
```

== Design Decisions

*Entity Storage*: `ESPHomeDevice` stores two structures: `entities` (vectors of entity metadata organized by type) and `entity_index_lut` (a lookup table mapping names/keys to indices). This structure optimizes lookup speed. You can store entity keys for repeated operations or store indices for consistent data access.

*Channels for State Updates*: When subscribing to state updates or logs, you provide a channel size and receive a `tokio::mpsc::Receiver<EntityStateUpdate>`. This fit naturally with Igloo's async architecture.

*Enum over Trait*: `ESPHomeDevice` stores an enum `AnyConnection` that can be either `Noise` or `Plain`. While I implemented a trait for connections, I chose the enum approach to avoid dynamic dispatch overhead from `dyn` trait objects.

= Outcome

This project unlocked ESPHome devices for use outside Home Assistant. The implementation served as the foundation for Igloo's ESPHome provider and demonstrated that third-party ESPHome integration was viable. The lessons learned from this implementation led to a cleaner, faster rewrite in Igloo V2.
