use std::io::{self, Cursor, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use uuid::Uuid;

use crate::RawPacket;

pub(crate) fn read_packet_content(mut reader: impl Read) -> io::Result<(i32, Vec<u8>)> {
    let id = reader.read_var_i32()?;
    let mut body = Vec::new();
    reader.read_to_end(&mut body)?;
    Ok((id, body))
}

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
        let mut result: i32 = 0;
        let mut len: u8 = 0;
        loop {
            let byte: u8 = self.read_u8()?;
            result |= i32::from(byte & 0x7F) << (7 * len);

            len += 1;
            if len > 5 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "exceeded maximum VarInt byte length",
                ));
            }

            if byte & 0x80 == 0 {
                break;
            }
        }
        return Ok(result);
    }

    /// Reads a signed 32 bit integer as a usize from the underlying reader, using variable-length encoding.
    fn read_var_i32_len(&mut self) -> io::Result<usize> {
        let value = self.read_var_i32()?;
        let value = usize::try_from(value)
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid VarInt value as length: {}", value))
            )?;
        Ok(value)
    }

    /// Reads a signed 64 bit integer from the underlying reader, using variable-length encoding.
    fn read_var_i64(&mut self) -> io::Result<i64> {
        let mut result: i64 = 0;
        let mut len: u8 = 0;
        loop {
            let byte: u8 = self.read_u8()?;
            result |= i64::from(byte & 0x7F) << (7 * len);

            len += 1;
            if len > 10 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "exceeded maximum VarLong byte length",
                ));
            }

            if byte & 0x80 == 0 {
                break;
            }
        }
        return Ok(result);
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
        let string = String::from_utf8(bytes)
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                "string has invalid utf8 characters",
            ))?;
        Ok(string)
    }

    /// Reads a single [packet][`RawPacket`] from the underlying reader.
    fn read_packet(&mut self) -> io::Result<RawPacket> {
        let len = self.read_var_i32_len()?;
        let mut content = vec![0; len];
        self.read_exact(&mut content)?;
        let (id, body) = read_packet_content(Cursor::new(content))?;
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

    /// Writes a usize as a signed 32 bit integer to the underlying writer, using variable-length encoding.
    fn write_var_i32_len(&mut self, v: usize) -> io::Result<()> {
        let v = i32::try_from(v)
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid VarInt value as length: {}", v),
            ))?;
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
        let len = packet.len();
        self.write_var_i32_len(len)?;
        self.write_var_i32(packet.id)?;
        self.write_all(&packet.body)?;
        Ok(())
    }
}

impl<T: Write> WriteMinecraftExt for T {}
