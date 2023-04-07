# `minecrevy_net`

A multi-layer implementation of the Minecraft networking protocol.

## Layers

`minecrevy_net` is split into the following layers:

- `RawServer`/`RawClient`: These only interact with `RawPacket`s.
- `ProtocolServer`/`ProtocolClient`: These work 