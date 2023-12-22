//! Minecraft protocol packet definitions common to multiple states.

use std::io;

use minecrevy_io::{args::IntArgs, McRead, McWrite};
use minecrevy_text::Text;

/// A packet sent by the server to indicate a failed login.
#[derive(Clone, PartialEq, Debug)]
pub struct Disconnect {
    /// The reason for the disconnect.
    pub reason: Text,
}

impl Default for Disconnect {
    fn default() -> Self {
        Self {
            reason: Text::from("Disconnected"),
        }
    }
}

impl McWrite for Disconnect {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.reason.write_default(writer)
    }
}

/// A packet sent by the server to indicate that the client should keep the connection alive.
#[derive(Clone, PartialEq, Debug)]
pub struct KeepAlive(pub i64);

impl McRead for KeepAlive {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self(i64::read(&mut reader, IntArgs { varint: false })?))
    }
}

impl McWrite for KeepAlive {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.0.write(&mut writer, IntArgs { varint: false })
    }
}

/// A packet sent by the server to indicate that the client should keep the connection alive.
#[derive(Clone, PartialEq, Debug)]
pub struct Ping(pub i32);

impl McRead for Ping {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self(i32::read(&mut reader, IntArgs { varint: false })?))
    }
}

impl McWrite for Ping {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.0.write(&mut writer, IntArgs { varint: false })
    }
}

/// A packet sent by the client to indicate that the client should keep the connection alive.
#[derive(Clone, PartialEq, Debug)]
pub struct PingRequest(pub i64);

impl McRead for PingRequest {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self(i64::read(&mut reader, IntArgs { varint: false })?))
    }
}

impl McWrite for PingRequest {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.0.write(&mut writer, IntArgs { varint: false })
    }
}
