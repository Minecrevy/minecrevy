use std::io;

use minecrevy_text::Text;

use crate::{options::StringOptions, McRead, McWrite};

/// In bytes.
const MAX_STRING_LENGTH: usize = 262144;

impl McRead for Text {
    type Options = ();

    fn read<R: io::Read>(reader: R, _: Self::Options) -> io::Result<Self> {
        let json = String::read(
            reader,
            StringOptions {
                max_len: Some(MAX_STRING_LENGTH),
            },
        )?;

        serde_json::from_str::<Text>(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Text {
    type Options = ();

    fn write<W: io::Write>(&self, writer: W, _: Self::Options) -> io::Result<()> {
        let json = serde_json::to_string::<Text>(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        json.write(
            writer,
            StringOptions {
                max_len: Some(MAX_STRING_LENGTH),
            },
        )
    }
}
