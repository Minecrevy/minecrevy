# Minecrevy

Disclaimer: This is primarily a research project to drive improvements to Bevy.

A toolkit for building [Rust](https://www.rust-lang.org/)-based
[Minecraft: Java Edition](https://www.minecraft.net/en-us) servers,
using [Bevy Engine](https://bevyengine.org/).
Minecrevy **is not** a Minecraft server; it's a framework for building one.

## Goals

- **Modular**: Build your server distribution exactly how you want.
- **Efficient**: Minecrevy-based servers should be able to scale into the 1000s of players.

## Non-Goals

### An out-of-the-box Solution

Minecrevy will not provide a production-ready Vanilla Minecraft server, but it
will provide the *tools* for a developer to make one.

### Dynamic Plugin Loading

Traditionally, Minecraft server software beyond Mojang's has provided some sort
of "dynamic plugin loading" (i.e. Spigot, Forge, Sponge, etc). Minecrevy has no
plans for this, instead focusing on streamlining static binary deployment.
Regardless, feel free to implement your own dynamic scripting on top.

## Roadmap

Minecrevy development is motivated through realistic usecases.
That means that for a new version of the Minecrevy toolkit to be released,
the project associated with the version must have reached its MVP milestone.

### List of projects

- `v0.2.0` - [Server list advertiser](projects/0.2.0-server-list/)
- `v0.3.0` - [Void lobby world](projects/0.3.0-void-lobby/)
- `v0.4.0` - [Lobby world](projects/0.4.0-lobby/)

## License

This code repository is dual-licensed under either:

- MIT License
- Apache License, Version 2.0

at your option.