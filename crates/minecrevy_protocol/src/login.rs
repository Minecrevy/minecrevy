//! Minecraft protocol packet definitions in the `Login` state.

use std::io;

use minecrevy_io::{
    args::{ListArgs, ListLength, OptionArgs, OptionTag, StringArgs},
    McRead, McWrite,
};
use uuid::Uuid;

/// A packet sent by the client to begin the login process.
#[derive(Clone, PartialEq, Debug)]
pub struct Start {
    /// The username of the player.
    pub username: String,
    /// The UUID of the player.
    pub uuid: Uuid,
}

impl McRead for Start {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            username: String::read(&mut reader, StringArgs { max_len: Some(16) })?,
            uuid: Uuid::read(reader, ())?,
        })
    }
}

/// A packet sent by the client to finish the login process.
#[derive(Clone, PartialEq, Debug)]
pub struct Acknowledged;

impl McRead for Acknowledged {
    type Args = ();

    fn read(_: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self)
    }
}

/// A packet sent by the server to indicate a successful login.
#[derive(Clone, PartialEq, Debug)]
pub struct Success {
    /// The UUID of the player.
    pub uuid: Uuid,
    /// The username of the player.
    pub username: String,
    /// The properties of the player, such as their skin.
    pub properties: Vec<Property>,
    /// Whether the client should immediately disconnect on packet processing errors.
    // TODO: Remove in 1.21.2
    pub strict_error_handling: bool,
}

impl McWrite for Success {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.uuid.write_default(&mut writer)?;
        self.username
            .write(&mut writer, StringArgs { max_len: Some(16) })?;
        self.properties.write(
            &mut writer,
            ListArgs {
                length: ListLength::VarInt,
                inner: (),
            },
        )?;
        self.strict_error_handling.write_default(writer)?;
        Ok(())
    }
}

/// A player profile property.
#[derive(Clone, PartialEq, Debug)]
pub struct Property {
    /// The name of the property.
    pub name: String,
    /// The value of the property.
    pub value: String,
    /// The encrypted signature of the property.
    pub signature: Option<String>,
}

impl McWrite for Property {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.name.write(
            &mut writer,
            StringArgs {
                max_len: Some(32767),
            },
        )?;
        self.value.write(
            &mut writer,
            StringArgs {
                max_len: Some(32767),
            },
        )?;
        self.signature.write(
            writer,
            OptionArgs {
                tag: OptionTag::Bool,
                inner: StringArgs {
                    max_len: Some(32767),
                },
            },
        )?;
        Ok(())
    }
}

/// A packet sent by the server to signal a failed login.
pub type Disconnect = crate::Disconnect;
