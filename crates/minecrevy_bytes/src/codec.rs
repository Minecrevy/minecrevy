use std::{io, sync::Arc};

use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::packet::RawPacket;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct CodecSettings {
    pub compression_threshold: Option<i32>,
    pub encryption_key: Option<[u8; 16]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RawPacketCodec {
    settings: Arc<CodecSettings>,
    compress: bool,
    encrypt: bool,
}

impl RawPacketCodec {
    pub fn new(settings: Arc<CodecSettings>) -> Self {
        Self {
            settings,
            compress: false,
            encrypt: false,
        }
    }

    pub fn enable_compression(&mut self) {
        self.compress = true;
    }

    pub fn enable_encryption(&mut self) {
        self.encrypt = true;
    }
}

impl Decoder for RawPacketCodec {
    type Item = RawPacket;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

impl Encoder<RawPacket> for RawPacketCodec {
    type Error = io::Error;

    fn encode(&mut self, packet: RawPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        packet.len()
        todo!()
    }
}
