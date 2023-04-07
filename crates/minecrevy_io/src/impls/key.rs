use std::io;

use minecrevy_core::key::Key;

use crate::{options::StringOptions, McRead, McWrite};

/// In bytes.
const MAX_KEY_LENGTH: usize = 32767;

impl McRead for Key {
    type Options = ();

    fn read<R: io::Read>(reader: R, _: Self::Options) -> io::Result<Self> {
        let string = String::read(
            reader,
            StringOptions {
                max_len: Some(MAX_KEY_LENGTH),
            },
        )?;

        string
            .parse::<Key>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Key {
    type Options = ();

    fn write<W: io::Write>(&self, writer: W, _: Self::Options) -> io::Result<()> {
        let string = self.to_string();

        string.write(
            writer,
            StringOptions {
                max_len: Some(MAX_KEY_LENGTH),
            },
        )
    }
}
