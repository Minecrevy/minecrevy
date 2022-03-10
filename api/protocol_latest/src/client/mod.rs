//! This module contains any packets sent by the client.

use minecrevy_protocol::PacketCodec;
pub use self::handshake::*;
pub use self::login::*;
pub use self::play::*;
pub use self::status::*;

mod handshake;
mod login;
mod play;
mod status;

pub fn codec() -> PacketCodec {
    let server = crate::server::codec();

    server.flip()
}
