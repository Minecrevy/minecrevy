use std::io::{self, Cursor, Read};

use byteorder::ReadBytesExt;
use tokio_util::{
    bytes::{Buf, BufMut, BytesMut},
    codec::{Decoder, Encoder},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RawPacket {
    pub id: i32,
    pub data: Vec<u8>,
}

pub struct RawPacketCodec {
    // TODO: Add compression support
    pub compression_threshold: Option<i32>,
}

impl Encoder<RawPacket> for RawPacketCodec {
    type Error = io::Error;

    fn encode(&mut self, item: RawPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = var_i32_size(item.id) + item.data.len();
        let len_i32 = i32::try_from(len)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "packet length too large"))?;
        dst.reserve(var_i32_size(len_i32) + len);
        write_var_i32(dst, len_i32);
        write_var_i32(dst, item.id);
        dst.put(item.data.as_ref());
        Ok(())
    }
}

impl Decoder for RawPacketCodec {
    type Item = RawPacket;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = Cursor::new(src.as_ref());
        match read_raw_packet(&mut cursor) {
            Ok(packet) => {
                src.advance(usize::try_from(cursor.position()).unwrap());
                Ok(Some(packet))
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => Err(e),
        }
    }
}

fn read_raw_packet(src: &mut Cursor<&[u8]>) -> io::Result<RawPacket> {
    let len = usize::try_from(read_var_i32(src)?)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid packet length"))?;

    let mut inner = Cursor::new(&src.get_ref()[src.position() as usize..][..len]);
    let id = read_var_i32(&mut inner)?;
    let mut data = Vec::new();
    inner.read_to_end(&mut data)?;

    Ok(RawPacket { id, data })
}

fn read_var_i32(src: &mut Cursor<&[u8]>) -> io::Result<i32> {
    const CONTINUE_BIT: u8 = 0x80;
    const SEGMENT_MASK: u8 = 0x7F;

    let readable = src.remaining();
    if readable == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "unexpected EOF",
        ));
    }

    let mut byte = src.read_u8()?;
    if (byte & CONTINUE_BIT) != CONTINUE_BIT {
        return Ok(byte as i32);
    }

    let max_read = 5.min(readable);
    let mut value = (byte & SEGMENT_MASK) as u32;

    let mut len = 1;
    while len < max_read {
        byte = src.read_u8()?;
        value |= ((byte & SEGMENT_MASK) as u32) << (len * 7);
        if (byte & CONTINUE_BIT) != CONTINUE_BIT {
            return Ok(value as i32);
        }
        len += 1;
    }

    if (len == 5) && (byte & CONTINUE_BIT == CONTINUE_BIT) {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "varint too large",
        ))
    } else {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "unexpected EOF",
        ))
    }
}

fn write_var_i32(dst: &mut BytesMut, value: i32) {
    const CONTINUE_BIT: u32 = 0x80;
    const SEGMENT_MASK: u32 = 0x7F;

    let value = value as u32;
    if (value & (0xFF_FF_FF_FF << 7)) == 0 {
        dst.put_u8(value as u8);
    } else if (value & (0xFF_FF_FF_FF << 14)) == 0 {
        let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 8) | (value >> 7);
        dst.put_u16(w as u16);
    } else if (value & (0xFF_FF_FF_FF << 21)) == 0 {
        let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 16)
            | (((value >> 7) & SEGMENT_MASK | CONTINUE_BIT) << 8)
            | (value >> 14);
        // write u24
        dst.put_slice(&w.to_be_bytes()[1..4]);
    } else if (value & (0xFF_FF_FF_FF << 28)) == 0 {
        let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 24)
            | (((value >> 7) & SEGMENT_MASK | CONTINUE_BIT) << 16)
            | (((value >> 14) & SEGMENT_MASK | CONTINUE_BIT) << 8)
            | (value >> 21);
        dst.put_u32(w);
    } else {
        let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 24)
            | (((value >> 7) & SEGMENT_MASK | CONTINUE_BIT) << 16)
            | (((value >> 14) & SEGMENT_MASK | CONTINUE_BIT) << 8)
            | ((value >> 21) & SEGMENT_MASK | CONTINUE_BIT);
        dst.put_u32(w);
        dst.put_u8((value >> 28) as u8);
    }
}

fn var_i32_size(value: i32) -> usize {
    static VAR_INT_LENGTHS: [usize; 33] = const {
        let mut lengths = [0; 33];
        let mut i: usize = 0;
        while i <= 32 {
            let sub = match i.checked_sub(1) {
                Some(x) => x,
                None => 0,
            };
            lengths[i] = (31 - sub).div_ceil(7);
            i += 1;
        }
        lengths[32] = 1; // Special case for the number 0.
        lengths
    };

    VAR_INT_LENGTHS[value.leading_zeros() as usize]
}

#[cfg(test)]
mod tests {
    use tokio_util::{
        bytes::BytesMut,
        codec::{Decoder, Encoder},
    };

    use crate::packet::RawPacketCodec;

    #[test]
    fn test_var_i32_size() {
        assert_eq!(super::var_i32_size(0), 1);
        assert_eq!(super::var_i32_size(1), 1);
        assert_eq!(super::var_i32_size(127), 1);
        assert_eq!(super::var_i32_size(128), 2);
        assert_eq!(super::var_i32_size(16383), 2);
        assert_eq!(super::var_i32_size(16384), 3);
        assert_eq!(super::var_i32_size(2097151), 3);
        assert_eq!(super::var_i32_size(2097152), 4);
        assert_eq!(super::var_i32_size(268435455), 4);
        assert_eq!(super::var_i32_size(268435456), 5);
        assert_eq!(super::var_i32_size(-1), 5);
        assert_eq!(super::var_i32_size(-268435456), 5);
    }

    #[test]
    fn test_read_var_i32() {
        let mut cursor = std::io::Cursor::new(&[0x00][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 0);

        let mut cursor = std::io::Cursor::new(&[0x01][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 1);

        let mut cursor = std::io::Cursor::new(&[0x7F][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 127);

        let mut cursor = std::io::Cursor::new(&[0x80, 0x01][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 128);

        let mut cursor = std::io::Cursor::new(&[0xFF, 0x7F][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 16383);

        let mut cursor = std::io::Cursor::new(&[0x80, 0x80, 0x01][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 16384);

        let mut cursor = std::io::Cursor::new(&[0xFF, 0xFF, 0x7F][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 2097151);

        let mut cursor = std::io::Cursor::new(&[0x80, 0x80, 0x80, 0x01][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 2097152);

        let mut cursor = std::io::Cursor::new(&[0xFF, 0xFF, 0xFF, 0x7F][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 268435455);

        let mut cursor = std::io::Cursor::new(&[0x80, 0x80, 0x80, 0x80, 0x01][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), 268435456);

        let mut cursor = std::io::Cursor::new(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F][..]);
        assert_eq!(super::read_var_i32(&mut cursor).unwrap(), -1);
    }

    #[test]
    fn test_write_var_i32() {
        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 0);
        assert_eq!(buf, BytesMut::from(&[0x00][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 1);
        assert_eq!(buf, BytesMut::from(&[0x01][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 127);
        assert_eq!(buf, BytesMut::from(&[0x7F][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 128);
        assert_eq!(buf, BytesMut::from(&[0x80, 0x01][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 16383);
        assert_eq!(buf, BytesMut::from(&[0xFF, 0x7F][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 16384);
        assert_eq!(buf, BytesMut::from(&[0x80, 0x80, 0x01][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 2097151);
        assert_eq!(buf, BytesMut::from(&[0xFF, 0xFF, 0x7F][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 2097152);
        assert_eq!(buf, BytesMut::from(&[0x80, 0x80, 0x80, 0x01][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 268435455);
        assert_eq!(buf, BytesMut::from(&[0xFF, 0xFF, 0xFF, 0x7F][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, 268435456);
        assert_eq!(buf, BytesMut::from(&[0x80, 0x80, 0x80, 0x80, 0x01][..]));

        let mut buf = BytesMut::new();
        super::write_var_i32(&mut buf, -1);
        assert_eq!(buf, BytesMut::from(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F][..]));
    }

    #[test]
    fn test_decode_raw_packet() {
        let mut codec = RawPacketCodec {
            compression_threshold: None,
        };
        let mut buf = BytesMut::from(&[0x03, 0x01, 0x02, 0x03][..]);
        let packet = codec.decode(&mut buf).unwrap().unwrap();
        assert_eq!(
            packet,
            super::RawPacket {
                id: 1,
                data: vec![0x02, 0x03],
            }
        );
    }

    #[test]
    fn test_encode_raw_packet() {
        let mut codec = RawPacketCodec {
            compression_threshold: None,
        };
        let mut buf = BytesMut::new();
        let packet = super::RawPacket {
            id: 1,
            data: vec![0x02, 0x03],
        };
        codec.encode(packet, &mut buf).unwrap();
        assert_eq!(buf, BytesMut::from(&[0x03, 0x01, 0x02, 0x03][..]));
    }
}
