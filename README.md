# Minecrevy

A toolkit for building [Rust](https://www.rust-lang.org/)-based
[Minecraft: Java Edition](https://www.minecraft.net/en-us) servers,
using [Bevy Engine](https://bevyengine.org/).
Minecrevy **is not** a Minecraft server; it's a framework for building one.

## Goals

- **Modular**: Build your server distribution exactly how you want.
- **Efficient**: Minecrevy-based servers should be able to scale into the 1000s of players.

## Non-Goals

### A Comprehensive Solution

Minecrevy will not provide a production-ready Vanilla Minecraft server, but it
will provide the *tools* for a developer to make one.

### Dynamic Plugin Loading

Traditionally, Minecraft server software beyond Mojang's has provided some sort
of "dynamic plugin loading" (i.e. Spigot, Forge, Sponge, etc). Minecrevy has no
plans for this, instead focusing on streamlining static binary deployment.
Regardless, feel free to implement your own dynamic scripting on top.

## Roadmap

Each `0.x` release milestone is focused on the ability to build a certain type
of server with the tools provided by Minecrevy.

### 0.1.0 - Ping Server

- [x] Multiplayer server list ping
    - [x] Automatic favicon resizing
- [x] Automatically disconnected if trying to login

### 0.2.0 - Void Lobby Server

- [ ] Login past loading screen, no chunks
- [ ] Basic chat support

### 0.3.0 - World Viewer Server

- [ ] Send chunks, read only
