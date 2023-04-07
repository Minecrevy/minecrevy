use std::io::{self, Read, Write};

use minecrevy_bytes::blocking::{ReadMinecraftExt, WriteMinecraftExt};

use crate::{
    options::{ListLength, ListOptions, StringOptions},
    McRead, McWrite,
};

impl McRead for String {
    type Options = StringOptions;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        // Read the len value and check upper bound
        let len = reader.read_var_i32_len()?;
        match options.max_len {
            Some(max_len) if len > max_len => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "exceeded max string length (max: {}, actual: {})",
                        max_len, len
                    ),
                ))
            }
            _ => {}
        }

        // Read the actual string as bytes
        let mut bytes = vec![0; len];
        reader.read_exact(&mut bytes)?;

        // Try to convert the bytes into valid UTF-8
        String::from_utf8(bytes).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "string has invalid UTF-8 characters",
            )
        })
    }
}

impl McWrite for String {
    type Options = StringOptions;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.max_len {
            Some(max_len) if self.len() > max_len => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "exceeded max string length (max: {}, actual: {})",
                        max_len,
                        self.len()
                    ),
                ))
            }
            _ => {}
        }

        writer.write_var_i32_len(self.len())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}

impl<T: McRead> McRead for Vec<T> {
    type Options = ListOptions<T::Options>;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        match options.length {
            ListLength::VarInt => {
                let len = reader.read_var_i32_len()?;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(T::read(&mut reader, options.inner.clone())?);
                }
                Ok(result)
            }
            ListLength::Byte => {
                let len = reader.read_i8()?;
                let len = usize::try_from(len).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid list length: {}", len),
                    )
                })?;
                let mut result = Vec::with_capacity(len);
                for _ in 0..len {
                    result.push(T::read(&mut reader, options.inner.clone())?);
                }
                Ok(result)
            }
            ListLength::Remaining => {
                let mut result = Vec::new();
                loop {
                    match T::read(&mut reader, options.inner.clone()) {
                        Ok(v) => result.push(v),
                        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(result)
            }
        }
    }
}

impl<T: McWrite> McWrite for Vec<T> {
    type Options = ListOptions<T::Options>;

    fn write<W: Write>(&self, mut writer: W, options: Self::Options) -> io::Result<()> {
        match options.length {
            ListLength::VarInt => writer.write_var_i32_len(self.len())?,
            ListLength::Byte => {
                let len = i8::try_from(self.len()).map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("exceeded maximum list length: {}", self.len()),
                    )
                })?;
                writer.write_i8(len)?;
            }
            ListLength::Remaining => { /* no length prefix since its inferred */ }
        }
        for element in self {
            element.write(&mut writer, options.inner.clone())?;
        }
        Ok(())
    }
}
