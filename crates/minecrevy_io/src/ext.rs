//! Extension traits for the builtin [`Read`] and [`Write`] traits.

use std::io::{self, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use uuid::Uuid;

use crate::{packet::RawPacket, util::varint_bytes};

/// Extends [Read] with methods for reading [Minecraft protocol data types][1].
///
/// **Note:** All methods here use [BigEndian].
///
/// [1]: https://wiki.vg/Protocol#Data_types
pub trait ReadMinecraftExt: Read {
    /// Reads an unsigned 8 bit integer as a boolean from the underlying reader.
    #[inline]
    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0x00)
    }

    /// Reads an unsigned 8 bit integer from the underlying reader.
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        ReadBytesExt::read_u8(self)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    #[inline]
    fn read_u16(&mut self) -> io::Result<u16> {
        ReadBytesExt::read_u16::<BigEndian>(self)
    }

    /// Reads an unsigned 32 bit integer from the underlying reader.
    #[inline]
    fn read_u32(&mut self) -> io::Result<u32> {
        ReadBytesExt::read_u32::<BigEndian>(self)
    }

    /// Reads an unsigned 64 bit integer from the underlying reader.
    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        ReadBytesExt::read_u64::<BigEndian>(self)
    }

    /// Reads an unsigned 128 bit integer from the underlying reader.
    #[inline]
    fn read_u128(&mut self) -> io::Result<u128> {
        ReadBytesExt::read_u128::<BigEndian>(self)
    }

    /// Reads a signed 8 bit integer from the underlying reader.
    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        ReadBytesExt::read_i8(self)
    }

    /// Reads a signed 16 bit integer from the underlying reader.
    #[inline]
    fn read_i16(&mut self) -> io::Result<i16> {
        ReadBytesExt::read_i16::<BigEndian>(self)
    }

    /// Reads a signed 32 bit integer from the underlying reader.
    #[inline]
    fn read_i32(&mut self) -> io::Result<i32> {
        ReadBytesExt::read_i32::<BigEndian>(self)
    }

    /// Reads a signed 64 bit integer from the underlying reader.
    #[inline]
    fn read_i64(&mut self) -> io::Result<i64> {
        ReadBytesExt::read_i64::<BigEndian>(self)
    }

    /// Reads a signed 128 bit integer from the underlying reader.
    #[inline]
    fn read_i128(&mut self) -> io::Result<i128> {
        ReadBytesExt::read_i128::<BigEndian>(self)
    }

    /// Reads a 32 bit floating point number from the underlying reader.
    #[inline]
    fn read_f32(&mut self) -> io::Result<f32> {
        ReadBytesExt::read_f32::<BigEndian>(self)
    }

    /// Reads a 64 bit floating point number from the underlying reader.
    #[inline]
    fn read_f64(&mut self) -> io::Result<f64> {
        ReadBytesExt::read_f64::<BigEndian>(self)
    }

    /// Reads a [Uuid] as an unsigned 128 bit integer from the underlying reader.
    #[inline]
    fn read_uuid(&mut self) -> io::Result<Uuid> {
        Ok(Uuid::from_u128(self.read_u128()?))
    }

    /// Reads a signed 32 bit integer from the underlying reader, using variable-length encoding.
    fn read_var_i32(&mut self) -> io::Result<i32> {
        pub const SEGMENT: u8 = 0b0111_1111;
        pub const CONTINUE: u8 = 0b1000_0000;

        let mut value = 0;
        let mut position = 0;

        loop {
            let byte = self.read_u8()?;
            value |= ((byte & SEGMENT) as i32) << position;
            position += 7;

            if position >= 32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt is too big",
                ))?;
            } else if byte & CONTINUE != CONTINUE {
                break;
            }
        }

        Ok(value)
    }

    /// Reads a signed 32 bit integer as a usize from the underlying reader, using variable-length encoding.
    fn read_var_i32_len(&mut self) -> io::Result<usize> {
        let value = self.read_var_i32()?;
        let value = usize::try_from(value).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid VarInt value as length: {}", value),
            )
        })?;
        Ok(value)
    }

    /// Reads a signed 64 bit integer from the underlying reader, using variable-length encoding.
    fn read_var_i64(&mut self) -> io::Result<i64> {
        pub const SEGMENT: u8 = 0b0111_1111;
        pub const CONTINUE: u8 = 0b1000_0000;

        let mut value: i64 = 0;
        let mut position: u8 = 0;

        loop {
            let byte = self.read_u8()?;
            value |= ((byte & SEGMENT) as i64) << position;
            position += 7;

            if position >= 64 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarLong is too big",
                ))?;
            } else if byte & CONTINUE != CONTINUE {
                break;
            }
        }

        Ok(value)
    }

    /// Reads a number of bytes from the underlying reader, with a variable-length 32 bit integer as the length prefix.
    fn read_bytes_var_i32(&mut self) -> io::Result<Vec<u8>> {
        let len = self.read_var_i32_len()?;
        let mut bytes = vec![0; len];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    /// Reads all remaining bytes from the underlying reader.
    fn read_bytes_remaining(&mut self) -> io::Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    /// Reads a [String] from the underlying reader, with a variable-length 32 bit integer as the length prefix.
    fn read_string(&mut self) -> io::Result<String> {
        let bytes = self.read_bytes_var_i32()?;
        let string = String::from_utf8(bytes).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "string has invalid utf8 characters",
            )
        })?;
        Ok(string)
    }

    /// Reads a single [packet][`RawPacket`] from the underlying reader.
    fn read_packet(&mut self) -> io::Result<RawPacket> {
        let len = self.read_var_i32_len()?;
        let mut reader = self.take(len as u64);

        let id = reader.read_var_i32()?;

        let remaining_len = len - varint_bytes(id);
        let mut body = Vec::with_capacity(remaining_len);
        reader.read_to_end(&mut body)?;

        Ok(RawPacket { id, body })
    }
}

impl<T: Read> ReadMinecraftExt for T {}

/// Extends [Write] with methods for writing [Minecraft protocol data types][1].
///
/// **Note:** All methods here use [BigEndian].
///
/// [1]: https://wiki.vg/Protocol#Data_types
pub trait WriteMinecraftExt: Write {
    /// Writes an unsigned 8 bit integer as a boolean to the underlying writer.
    fn write_bool(&mut self, v: bool) -> io::Result<()> {
        self.write_u8(if v { 0x01 } else { 0x00 })
    }

    /// Writes an unsigned 8 bit integer to the underlying writer.
    #[inline]
    fn write_u8(&mut self, v: u8) -> io::Result<()> {
        WriteBytesExt::write_u8(self, v)
    }

    /// Writes an unsigned 16 bit integer to the underlying writer.
    #[inline]
    fn write_u16(&mut self, v: u16) -> io::Result<()> {
        WriteBytesExt::write_u16::<BigEndian>(self, v)
    }

    /// Writes an unsigned 32 bit integer to the underlying writer.
    #[inline]
    fn write_u32(&mut self, v: u32) -> io::Result<()> {
        WriteBytesExt::write_u32::<BigEndian>(self, v)
    }

    /// Writes an unsigned 64 bit integer to the underlying writer.
    #[inline]
    fn write_u64(&mut self, v: u64) -> io::Result<()> {
        WriteBytesExt::write_u64::<BigEndian>(self, v)
    }

    /// Writes an unsigned 128 bit integer to the underlying writer.
    #[inline]
    fn write_u128(&mut self, v: u128) -> io::Result<()> {
        WriteBytesExt::write_u128::<BigEndian>(self, v)
    }

    /// Writes a signed 8 bit integer to the underlying writer.
    #[inline]
    fn write_i8(&mut self, v: i8) -> io::Result<()> {
        WriteBytesExt::write_i8(self, v)
    }

    /// Writes a signed 16 bit integer to the underlying writer.
    #[inline]
    fn write_i16(&mut self, v: i16) -> io::Result<()> {
        WriteBytesExt::write_i16::<BigEndian>(self, v)
    }

    /// Writes a signed 32 bit integer to the underlying writer.
    #[inline]
    fn write_i32(&mut self, v: i32) -> io::Result<()> {
        WriteBytesExt::write_i32::<BigEndian>(self, v)
    }

    /// Writes a signed 64 bit integer to the underlying writer.
    #[inline]
    fn write_i64(&mut self, v: i64) -> io::Result<()> {
        WriteBytesExt::write_i64::<BigEndian>(self, v)
    }

    /// Writes a signed 128 bit integer to the underlying writer.
    #[inline]
    fn write_i128(&mut self, v: i128) -> io::Result<()> {
        WriteBytesExt::write_i128::<BigEndian>(self, v)
    }

    /// Writes a 32 bit floating point number to the underlying writer.
    #[inline]
    fn write_f32(&mut self, v: f32) -> io::Result<()> {
        WriteBytesExt::write_f32::<BigEndian>(self, v)
    }

    /// Writes a 64 bit floating point number to the underlying writer.
    #[inline]
    fn write_f64(&mut self, v: f64) -> io::Result<()> {
        WriteBytesExt::write_f64::<BigEndian>(self, v)
    }

    /// Writes a [Uuid] as an unsigned 128 bit integer to the underlying writer.
    #[inline]
    fn write_uuid(&mut self, v: Uuid) -> io::Result<()> {
        self.write_u128(v.as_u128())
    }

    /// Writes a signed 32 bit integer to the underlying writer, using variable-length encoding.
    fn write_var_i32(&mut self, v: i32) -> io::Result<()> {
        pub const MASK: u32 = 0xFF_FF_FF_FF;
        pub const SEGMENT: u32 = 0b0111_1111;
        pub const CONTINUE: u32 = 0b1000_0000;

        let value = v as u32;

        if (value & (MASK << 7)) == 0 {
            self.write_u8(value as u8)?;
        } else if (value & (MASK << 14)) == 0 {
            let w = (value & SEGMENT | CONTINUE) << 8 | (value >> 7);
            self.write_u16(w as u16)?;
        } else if (value & (MASK << 21)) == 0 {
            let w = (value & SEGMENT | CONTINUE) << 16
                | ((value >> 17) & SEGMENT | CONTINUE) << 8
                | (value >> 14);
            self.write_u24::<BigEndian>(w)?;
        } else if (value & (MASK << 28)) == 0 {
            let w = (value & SEGMENT | CONTINUE) << 24
                | ((value >> 7) & SEGMENT | CONTINUE) << 16
                | ((value >> 14) & SEGMENT | CONTINUE) << 8
                | (value >> 21);
            self.write_u32(w)?;
        } else {
            let w = (value & SEGMENT | CONTINUE) << 24
                | ((value >> 7) & SEGMENT | CONTINUE) << 16
                | ((value >> 14) & SEGMENT | CONTINUE) << 8
                | ((value >> 21) & SEGMENT | CONTINUE);
            self.write_u32(w)?;
            self.write_u8((value >> 28) as u8)?;
        }

        Ok(())
    }

    /// Writes a usize as a signed 32 bit integer to the underlying writer, using variable-length encoding.
    fn write_var_i32_len(&mut self, v: usize) -> io::Result<()> {
        let v = i32::try_from(v).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid VarInt value as length: {}", v),
            )
        })?;
        self.write_var_i32(v)
    }

    /// Writes a signed 64 bit integer to the underlying writer, using variable-length encoding.
    fn write_var_i64(&mut self, v: i64) -> io::Result<()> {
        let mut v = v as u32;
        loop {
            if (v & !0x7F) == 0 {
                self.write_u8(v as u8)?;
                return Ok(());
            }

            self.write_u8(((v & 0x7F) | 0x80) as u8)?;
            v >>= 7;
        }
    }

    /// Writes a number of bytes to the underlying writer, with a variable-length 32 bit integer as the length prefix.
    fn write_bytes_var_i32(&mut self, v: &[u8]) -> io::Result<()> {
        self.write_var_i32_len(v.len())?;
        self.write_all(v)
    }

    /// Writes all bytes to the underlying writer.
    #[inline]
    fn write_bytes_remaining(&mut self, v: &[u8]) -> io::Result<()> {
        self.write_all(v)
    }

    /// Writes a [String] to the underlying writer, with a variable-length 32 bit integer as the length prefix.
    #[inline]
    fn write_string(&mut self, v: &str) -> io::Result<()> {
        self.write_bytes_var_i32(v.as_bytes())
    }

    /// Writes a single [packet][`RawPacket`] to the underlying writer.
    fn write_packet(&mut self, packet: &RawPacket) -> io::Result<()> {
        self.write_var_i32_len(packet.len())?;
        self.write_var_i32(packet.id)?;
        self.write_all(&packet.body)?;
        Ok(())
    }
}

impl<T: Write> WriteMinecraftExt for T {}
