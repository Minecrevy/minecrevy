use std::io::{self, Read, Write};

use flexstr::SharedStr;

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};

use crate::{McRead, McWrite, StringOptions};

impl McRead for SharedStr {
    type Options = StringOptions;

    fn read<R: Read>(mut reader: R, options: StringOptions) -> std::io::Result<Self> {
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
        let string = String::from_utf8(bytes).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "string has invalid UTF-8 characters",
            )
        })?;

        Ok(SharedStr::from(string))
    }
}

impl McWrite for SharedStr {
    type Options = StringOptions;

    fn write<W: Write>(&self, mut writer: W, options: StringOptions) -> io::Result<()> {
        match options.max_len {
            Some(max_len) if self.len() > max_len => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("exceeded max string length (max: {}, actual: {})", max_len, self.len()),
            )),
            _ => {}
        }

        writer.write_var_i32_len(self.len())?;
        writer.write_all(self.as_bytes())?;
        Ok(())
    }
}
