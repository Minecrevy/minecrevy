//! Minecraft protocol packet definitions in the `Handshake` state.

use std::io;

use minecrevy_io::{
    args::{IntArgs, StringArgs},
    McRead,
};

/// A packet sent by the client to initiate a connection.
#[derive(Clone, PartialEq, Debug)]
pub struct Handshake {
    /// The protocol version of the client.
    pub protocol_version: i32,
    /// The address of the server the client is connecting to.
    pub server_address: String,
    /// The port of the server the client is connecting to.
    pub server_port: u16,
    /// The next state the client wants to transition to.
    ///
    /// `1` for status, `2` for login.
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
