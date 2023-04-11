# OUTDATED - Minecrevy Design Document

This document outlines the architecture design of *Minecrevy*, a Minecraft server software project written in Rust, using the Bevy game engine.

## Provided Crates

*Minecrevy* is split into many individual crates to encourage decoupled and coherent design.

- `minecrevy_core` (TODO) - Provides common utilities and infrastructure.
- `minecrevy_daemon` (TODO) - Provides a daemon to run *Minecrevy* as a true OS service.
- `minecrevy_io` (TODO) - Provides I/O types and traits for the Minecraft protocol.
    - Depends on `minecrevy_core`.
- `minecrevy_protocol` (TODO) - Provides Packet types for the version of the Minecraft protocol that *Minecrevy* presently supports.
    - Depends on `minecrevy_io`.
- `minecrevy_text` (TODO) - Provides Minecraft chat related types and utilities.
    - Depends on `minecrevy_core`.
- `minecrevy_tui` (TODO) - Provides a terminal interface.
- `minecrevy_worldgen` (TODO) - Provides Vanilla-like Minecraft world generation facilities.

## Depended Crates

Regarding *Minecrevy*'s use of the Bevy game engine, the following crates are used and may be of interest to others:

- `bevy_app`
- `bevy_core`
- `bevy_ecs`
- `bevy_math`
- `bevy_reflect`
- `bevy_tasks`
- `beef` - More compact Copy-on-write type than Rust's builtin `Cow` type.
- `serde`