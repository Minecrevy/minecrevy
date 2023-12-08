# minecrevy_net

A [Minecraft: Java Edition](https://www.minecraft.net/en-us) compatible [networking](https://wiki.vg/Protocol) library, powered by [Bevy](https://bevyengine.org/) and [Tokio](https://tokio.rs/).

## Requirements

These requirements are a must to be compatible with the Minecraft: Java Edition protocol.

- Big-Endian over the network.
- Client-Server architecture.

## Goals

- **Embrace ECS**: Client connections are stored as Entities. Connections are dropped when the entity is despawned.
- **Performant**: Scale up to many connections without breaking a sweat.
- **Flexible**: Handling N clients should be similarly easy to handling 1 client.
- **Comprehensive**: Support server-side networking, but also client-side networking.

## Prior Art

Bevy ecosystem networking libraries:

- [bevy_replicon](https://github.com/lifescapegame/bevy_replicon)
- [matchbox](https://github.com/johanhelsing/matchbox)
- [bevy_renet](https://github.com/lucaspoffo/renet/tree/master/bevy_renet)
- [bevy_client_server_events](https://github.com/edouardpoitras/bevy_client_server_events)
- [bevy_quinnet](https://github.com/Henauxg/bevy_quinnet)
