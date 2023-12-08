# minecrevy_io

A library for reading and writing Minecraft protocol types.

## Example without derive

```rust
use std::io;
use minecrevy_io::{
    args::{IntArgs, StringArgs},
    McRead,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

impl McRead for Handshake {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            protocol_version: i32::read(&mut reader, IntArgs { varint: true })?,
            server_address: String::read(&mut reader, StringArgs { max_len: Some(255) })?,
            server_port: u16::read(&mut reader, ())?,
            next_state: i32::read(&mut reader, IntArgs { varint: true })?,
        })
    }
}
```