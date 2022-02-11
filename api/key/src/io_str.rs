use std::io::{self, Read, Write};

use minecrevy_io_str::{McRead, McWrite};
use minecrevy_io_str::StringOptions;

use crate::Key;

const KEY_MAX_LEN: usize = 32767;

impl McRead for Key {
    type Options = ();

    fn read<R: Read>(reader: R, _options: Self::Options) -> std::io::Result<Self> {
        let mut options = StringOptions::default();
        options.max_len = Some(KEY_MAX_LEN);

        let str = String::read(reader, options)?;

        Key::parse(str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Key {
    type Options = ();

    fn write<W: Write>(&self, writer: W, _options: Self::Options) -> io::Result<()> {
        let mut options = StringOptions::default();
        options.max_len = Some(KEY_MAX_LEN);

        let str = self.to_string();

        str.write(writer, options)
    }
}
