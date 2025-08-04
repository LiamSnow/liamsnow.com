---
title: esphomebridge-rs
desc: ESPHome client API written in Rust
date: 2025-01-15
homepage: true
---

[GitHub Repo](https://github.com/liamsnow/esphomebridge-rs)

Before I started working on [Igloo](https://liamsnow.com/igloo)
I knew that I was going to need a Rust interface for ESPHome devices.
This was quite a difficult project, but I had a lot of fun working on it.

This project is not exactly [aioesphomeapi](https://github.com/LiamSnow/esphomebridge-rs/blob/main/github.com/esphome/aioesphomeapi)
(made by the creators of ESPHome and used in Home Assistant)
rewritten in Rust. It's built to fit into the architecture of Igloo
and serve as an example to implement other providers.

There had been a little bit of work,
[pixelspark](https://github.com/pixelspark) had made [esphome-rs](https://github.com/pixelspark/esphome-rs) but it didn't really implement that much.
Luckily, ESPHome was made with protobuf, which I used to automatically generate
a bunch of the interface code.

However, the hardest part here is working with noise connections.
This seems to be a default for new ESPHome devices (including all the ones
I had). I wasn't able to find much documentation for it, but after digging through
aioesphomeapi for long enough I was able to reverse-engineer it.

## Brief ESPHome Overview
### Noise Protocol

Luckily I was able to offload a lot of the work to the
[snow](https://crates.io/crates/snow) which I just had to plugin
the params `Noise_NNpsk0_25519_ChaChaPoly_SHA256` and prologue
`NoiseAPIInit\x00\x00`.

Then you have do to the noise handshake, which just entails
sending hello `0x01`, waiting for a hello back, and then
waiting for the handshake to complete.

Then from there on out, it's just like working with a normal/non-noise connection.

### Protobuf
After completing all connection setup, then comes the protobuf protocol.

Here you introduce yourself as the client, with your name and supported versions.
Next, you make a connection request, which will usually includes a password
unless its a noise connection.

After connecting, its standard to fetch all entities and services. Services
can be scripts you put on the device and some things with voice assistants.
Everything else here is an entity.

On side note, I feel like ESPHome's naming scheme is quite weird. In my mind
a device should be an entity and what they call entities (light, switch, number, etc.)
should be components.

## My Implementation

I played around with a bunch of ways to do this, but landed on something
I'm pretty happy with.

Here is what the end result looks like:

Connect:
```rust
let dev = Device::new_noise("IP", "NOISE_PSK")?;
dev.connect().await?;
```

Print all buttons:
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
        state: true
        ..Default::default()
    };

    dev.light_command(req).await?;
}
```

Turn on a light, given the entity name
```rust
let req = api::LightCommandRequest {
    key: dev.get_light_key_from_name("rgbct_bulb")?,
    has_state: true,
    state: true
    ..Default::default()
};

dev.light_command(req).await?;
```

### Entities
Theres some big, very opinated decisions I made for this crates around entities.
Looking for the best performance, I decided that `ESPHomeDevice` would store
two things for entities: `entities` and `entity_index_lut`.

`entities` stores vectors of entity metadata (key, name, etc).
It has a field for each entity type (ex. `entities.light`, `entities.switch`).

`entity_index_lut` is a look-up table to find the index in `entities` given
a name or key. This way, given a key you can get the name of the entity OR
given a name you can the key. 

I found this structure to be both very versatile and fast. If you want to perform
actions on entities repeatidly, you can just store their key. If you want to
consistently lookup an entities data, you can just store the index.


### Smaller Decisions
Theres also some smaller interesting decisions I made:
 1. **Use of channels**: If you subscribe to state updates or logs for a device, you put in a channel size and get back a `tokio::mpsc::Reciever<EntityStateUpdate>`. While I totally understand other people wanting a callback, I found this method to make the most sense for most applications, especially Igloo.
 2. **Enum over Trait**: I decided to have `ESPHomeDevice` store an enum `AnyConnection` which can be either `Noise` or `Plain`. Funny enough I actually do have a trait for connections, but I decided to go down this route to avoid using `dyn`.
