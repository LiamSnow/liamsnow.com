---
title: Making Igloo's provider system
desc: How Igloo securely runs and communicates with extensions
date: 2025-09-20
homepage: true
---

If you don't know what Igloo is please check out the [Project Page](../projects/igloo).

For this article I will refer to the following:
 - _Provider_: Provides integration to smart home platforms like ESPHome, Apple HomeKit, MQTT, etc.
 - _Extension_: a superset of a provider. Can provide other/more functionality like custom dashboard elements.

# Objectives
In order of priority:
 1. Reliable
 2. Backwards-compatible
 3. Secure
 4. Fast

# How to Support Providers

## Monolithic
Build everything into the core system.

**Pros**:
 - No IPC (inter-process communication): Makes it simpler and faster
 - Smaller binary size: instead of having 20 Rust binaries all with their own `tokio`, `serde`, etc. you get one optimized binary

**Cons**:
 - Updating any provider requires a full restart
 - Slower build times
 - No containment: Each provider has full permission to do anything. Furthermore, a crash in any provider crashes the entire system.
 - Requires maintenance of a giant monorepo

## Out-of-Process
Each provider is its own separate process.

**Pros**:
 - Contained: A crash in a provider only takes out that provider
 - Secure: Using [Landlock](https://docs.kernel.org/userspace-api/landlock.html) and IPTables we can enforce strict permissions for each provider
 - Rolling Updates: We can update providers without a full system restart

**Cons**:
 - Requires IPC which is both slower and more complex
 - Requires a package manager

## Final Decision
Given our objectives, the out-of-process architecture is the clear winner. Reliability and security take priority over the performance cost of IPC.

# Out-of-Process Implementation
The out-of-process architecture still leaves several options:
 1. Run as a Linux process
 2. Run in a [Docker](https://www.docker.com/) container
 3. Run as [WASM](https://webassembly.org/) process

The WASM approach was initially appealing. Binaries can be built once for any platform, we can enforce very strict permissions, and we can interface with them very directly (no separate process). However, WASM isn't ready yet. Python on WASM has immature tooling. Required packages don't work reliably and [WASI (WebAssembly System Interface)](https://wasi.dev/) hardware access is still in development. WASM will likely be viable in the future, but it's not the right choice today.

Given that many Linux machines now support [Landlock](https://docs.kernel.org/userspace-api/landlock.html), I chose Linux processes over Docker containers. Docker would add complexity and resource overhead for minimal benefit.

**Security Approach:**
- **Landlock**: Linux Security Module that restricts filesystem access. Each extension can only access files in its own directory.
- **IPTables**: Restricts network access, blocking LAN and/or WAN connections as needed.

This provides defense-in-depth: even if an extension is compromised, its access to the system is strictly limited.

# IPC

I explored several options including [Protocol Buffers](https://protobuf.dev/), [FlatBuffers](https://flatbuffers.dev/), and [Cap'n Proto](https://capnproto.org/). While these technologies handle schema evolution and backwards compatibility well, they have significant ergonomics issues. The generated code lacks utility functions, constructors, and trait implementations. It doesn't follow Rust idioms.For example, if you have 20 commands that all share `key` and `name` fields, the generated code doesn't group them or provide traits. The code generation also adds complexity with separate schema files.

[PyO3](https://pyo3.rs) offers a better approach. With PyO3, we can write a Rust library that Python code can use directly, and it generates Python type stubs for us. This eliminates code generation complexity while maintaining type safety.

Given this, I built a custom protocol using length-delimited Bincode over Unix domain sockets. Since the communication is Rust-to-Rust (with Python using PyO3 bindings), a simple binary protocol works well.

The protocol includes a few core message types:
 1. Create Device (from provider) & Device Created (from Igloo)
 2. Register Entity (from provider)
 3. Write component
    - From a provider: the device has updated and these changes should be applied to the device tree
    - From Igloo: requesting a component change, typically converted to a hardware request by the provider
 4. Custom commands specific to each provider

For serialization, I evaluated [Bincode](https://lib.rs/crates/bincode) and [Borsh](https://borsh.io/). Benchmarking a typical message with [Criterion](https://lib.rs/crates/criterion) (serialization/deserialization round-trip):
 - Borsh: 15ns
 - Bincode: 30ns
 - JSON: 350ns

I initially used Borsh, but switched to Bincode because Borsh limits enums to 64 variants (it uses u8 discriminants but enforces a 64-item cap). This would restrict us to only 64 components.

## Backwards-Compatibility
Bincode doesn't have built-in schema evolution like Protocol Buffers. We implement backwards compatibility manually with these constraints:
 1. Enforce append-only (no modifications, no deletions) to `components.toml` and the `Message` enum
 2. Bincode doesn't support custom discriminants. We work around this by sorting the components enum and always adding new variants at the bottom. New `Message` variants are also appended.
 3. Providers send their maximum supported component ID and message ID. Igloo never sends messages or components exceeding these limits.

This approach provides forward compatibility: old providers ignore new components and messages they don't understand.
