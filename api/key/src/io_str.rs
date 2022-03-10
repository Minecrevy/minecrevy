use std::io::{self, Read, Write};

use minecrevy_io_str::{McRead, McWrite};
use minecrevy_io_str::StringOptions;

use crate::{Key, KeyRef};

/// Configurable options for parsing [`Key`]s in the Minecraft protocol.
#[derive(Clone, Debug)]
pub struct KeyOptions {
    /// Specifies that the encoded/decoded string should not exceed the specified length.
    /// The default max length is 32,767 bytes.
    ///
    /// Setting this option to [`None`] simply means there is no length checking.
    pub max_len: Option<usize>,
}

impl Default for KeyOptions {
    fn default() -> Self {
        Self {
            max_len: Some(32767)
        }
    }
}

impl McRead for Key {
    type Options = KeyOptions;

    fn read<R: Read>(reader: R, options: Self::Options) -> std::io::Result<Self> {
        let options = StringOptions {
            max_len: options.max_len
        };

        let str = String::read(reader, options)?;

        Key::parse(str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Key {
    type Options = KeyOptions;

    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()> {
        let options = StringOptions {
            max_len: options.max_len
        };

        let str = self.to_string();

        str.write(writer, options)
    }
}

impl<'a> McWrite for KeyRef<'a> {
    type Options = KeyOptions;

    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()> {
        let options = StringOptions {
            max_len: options.max_len
        };

        let str = self.to_string();

        str.write(writer, options)
    }
}
