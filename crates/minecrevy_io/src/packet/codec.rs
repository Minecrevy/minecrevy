use std::{
    io::{self, Cursor},
    sync::Arc,
};

use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::{
    packet::RawPacket,
    std_ext::{ReadMinecraftExt, WriteMinecraftExt},
};

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

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO: compression & encryption
        let mut cursor = Cursor::<&[u8]>::new(&src);
        match cursor.read_packet() {
            Ok(packet) => {
                // reading was successful, advance the outer buffer and return
                let position = usize::try_from(cursor.position()).unwrap();
                src.advance(position);
                Ok(Some(packet))
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl Encoder<RawPacket> for RawPacketCodec {
    type Error = io::Error;

    fn encode(&mut self, packet: RawPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // TODO: compression & encryption
        let mut bytes = Vec::new();
        bytes.write_packet(&packet)?;
        dst.extend_from_slice(&bytes);
        Ok(())
    }
}
