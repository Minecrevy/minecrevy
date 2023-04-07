use std::{io, sync::Arc};

use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::packet::RawPacket;

/// Settings for the [`RawPacketCodec`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct CodecSettings {
    /// How large a [`RawPacket`] needs to be before it's compressed.
    pub compression_threshold: Option<i32>,
    /// The public key used to encrypt the [`RawPacket`].
    pub encryption_key: Option<[u8; 16]>,
}

/// A [`Decoder`] and [`Encoder`] that handles [`RawPacket`]s.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RawPacketCodec {
    settings: Arc<CodecSettings>,
    compress: bool,
    encrypt: bool,
}

impl RawPacketCodec {
    /// Constructs a new codec with the given settings.
    pub fn new(settings: Arc<CodecSettings>) -> Self {
        Self {
            settings,
            compress: false,
            encrypt: false,
        }
    }

    /// Enables compression for the codec.
    pub fn enable_compression(&mut self) {
        self.compress = true;
    }

    /// Enables encryption for the codec.
    pub fn enable_encryption(&mut self) {
        self.encrypt = true;
    }
}

impl Decoder for RawPacketCodec {
    type Item = RawPacket;
    type Error = io::Error;

    fn decode(&mut self, _src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

impl Encoder<RawPacket> for RawPacketCodec {
    type Error = io::Error;

    fn encode(&mut self, _packet: RawPacket, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}
