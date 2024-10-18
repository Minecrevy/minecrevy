//! Minecraft protocol packet definitions in the `Config` state.

use std::io;

use minecrevy_io::{
    args::{IntArgs, ListArgs, ListLength, StringArgs},
    McRead, McWrite,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A packet sent by both the client and the server to inform each other
/// of the data packs that they know about.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KnownDataPacks {
    /// The data packs that the server knows about.
    pub packs: Vec<DataPack>,
}

impl McRead for KnownDataPacks {
    type Args = ();

    fn read(reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        let packs = Vec::<DataPack>::read(
            reader,
            ListArgs {
                length: ListLength::VarInt,
                inner: (),
            },
        )?;
        Ok(Self { packs })
    }
}

impl McWrite for KnownDataPacks {
    type Args = ();

    fn write(&self, writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.packs.write(
            writer,
            ListArgs {
                length: ListLength::VarInt,
                inner: (),
            },
        )
    }
}

/// A data pack that the server knows about.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DataPack {
    /// The namespace of the data pack.
    pub namespace: String,
    /// The ID of the data pack.
    pub id: String,
    /// The version of the data pack.
    pub version: String,
}

impl McRead for DataPack {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            namespace: String::read_default(&mut reader)?,
            id: String::read_default(&mut reader)?,
            version: String::read_default(reader)?,
        })
    }
}

impl McWrite for DataPack {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.namespace.write_default(&mut writer)?;
        self.id.write_default(&mut writer)?;
        self.version.write_default(writer)
    }
}

/// A packet sent by the server to fill the client's registry.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RegistryData<T> {
    /// The ID of the registry.
    pub registry_id: String,
    /// The entries in the registry.
    pub entries: Vec<RegistryDataEntry<T>>,
}

impl<T: DeserializeOwned> McRead for RegistryData<T> {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            registry_id: String::read(
                &mut reader,
                StringArgs {
                    max_len: Some(32767),
                },
            )?,
            entries: Vec::<RegistryDataEntry<T>>::read(
                reader,
                ListArgs {
                    length: ListLength::VarInt,
                    inner: (),
                },
            )?,
        })
    }
}

impl<T: Serialize> McWrite for RegistryData<T> {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.registry_id.write(
            &mut writer,
            StringArgs {
                max_len: Some(32767),
            },
        )?;
        self.entries.write(
            writer,
            ListArgs {
                length: ListLength::VarInt,
                inner: (),
            },
        )
    }
}

/// An entry in [`RegistryData`].
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RegistryDataEntry<T> {
    /// The ID of the entry.
    pub entry_id: String,
    /// The data of the entry, if any.
    #[serde(default = "none", skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

fn none<T>() -> Option<T> {
    None
}

impl<T: DeserializeOwned> McRead for RegistryDataEntry<T> {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            entry_id: String::read(
                &mut reader,
                StringArgs {
                    max_len: Some(32767),
                },
            )?,
            data: {
                let tag = bool::read(&mut reader, ())?;
                if tag {
                    Some(
                        fastnbt::from_reader(reader)
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
                    )
                } else {
                    None
                }
            },
        })
    }
}

impl<T: Serialize> McWrite for RegistryDataEntry<T> {
    type Args = ();

    fn write(&self, mut writer: impl io::Write, (): Self::Args) -> io::Result<()> {
        self.entry_id.write(
            &mut writer,
            StringArgs {
                max_len: Some(32767),
            },
        )?;
        self.data.is_some().write(&mut writer, ())?;
        if let Some(data) = &self.data {
            fastnbt::to_writer(writer, data)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        }
        Ok(())
    }
}

/// A packet sent by the server to signal a failed configuration process.
pub type Disconnect = crate::Disconnect;

/// A packet sent by the server to signal the end of the configuration process.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Finish;

impl McWrite for Finish {
    type Args = ();

    fn write(&self, _: impl io::Write, _: Self::Args) -> io::Result<()> {
        Ok(())
    }
}

/// General information about the client.
#[derive(Clone, PartialEq, Debug)]
pub struct ClientInformation {
    /// The locale of the client, e.g. `en_US`.
    pub locale: String,
    /// The view distance of the client.
    pub view_distance: i8,
    /// The chat mode of the client.
    /// 0: enabled, 1: commands only, 2: hidden.
    pub chat_mode: i32,
    /// Whether chat colors are enabled.
    pub chat_colors: bool,
    /// The displayed skin parts of the client.
    pub displayed_skin_parts: u8,
    /// The main hand of the client.
    /// 0: left, 1: right.
    pub main_hand: i32,
    /// Whether text filtering is enabled.
    pub enable_text_filtering: bool,
    /// Whether server listings are allowed.
    pub allow_server_listings: bool,
}

impl McRead for ClientInformation {
    type Args = ();

    fn read(mut reader: impl io::Read, (): Self::Args) -> io::Result<Self> {
        Ok(Self {
            locale: String::read(&mut reader, StringArgs { max_len: Some(16) })?,
            view_distance: i8::read(&mut reader, ())?,
            chat_mode: i32::read(&mut reader, IntArgs { varint: true })?,
            chat_colors: bool::read(&mut reader, ())?,
            displayed_skin_parts: u8::read(&mut reader, ())?,
            main_hand: i32::read(&mut reader, IntArgs { varint: true })?,
            enable_text_filtering: bool::read(&mut reader, ())?,
            allow_server_listings: bool::read(reader, ())?,
        })
    }
}

/// A packet sent by the client to acknowledge the end of the configuration process.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct AcknowledgeFinish;

impl McRead for AcknowledgeFinish {
    type Args = ();

    fn read(_: impl io::Read, _: Self::Args) -> io::Result<Self> {
        Ok(Self)
    }
}
