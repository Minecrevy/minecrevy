use std::io::{self, Read, Write};

use minecrevy_io_str::{McRead, McWrite};
use minecrevy_io_str::StringOptions;

use crate::Text;

const TEXT_MAX_LEN: usize = 262144;

impl McRead for Text {
    type Options = ();

    fn read<R: Read>(reader: R, _options: Self::Options) -> io::Result<Self> {
        let mut options = StringOptions::default();
        options.max_len = Some(TEXT_MAX_LEN);

        let str = String::read(reader, options)?;
        Text::from_json_string(&str)
    }
}

impl McWrite for Text {
    type Options = ();

    fn write<W: Write>(&self, writer: W, _options: Self::Options) -> io::Result<()> {
        let mut options = StringOptions::default();
        options.max_len = Some(TEXT_MAX_LEN);

        let str = self.to_json_string();
        str.write(writer, options)
    }
}
