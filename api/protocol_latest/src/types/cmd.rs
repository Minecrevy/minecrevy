use std::io::{self, Read, Write};

use minecrevy_io_buf::ReadMinecraftExt;
use minecrevy_io_str::{IntOptions, ListLength, ListOptions, McRead, McWrite, StringOptions};
use minecrevy_key::{Key, KeyOptions, KeyRef};

#[derive(Clone, PartialEq, Debug)]
pub struct CommandNode {
    pub executable: bool,
    pub children: Vec<i32>,
    pub redirect: Option<i32>,
    pub value: CommandNodeValue,
}

impl McRead for CommandNode {
    type Options = ();

    fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
        let flags = reader.read_u8()?;
        Ok(Self {
            executable: flags & 0x04 == 0x04,
            children: Vec::read(&mut reader, ListOptions {
                length: ListLength::VarInt,
                inner: IntOptions::varint(),
            })?,
            redirect: if flags & 0x08 == 0x08 {
                Some(i32::read(&mut reader, IntOptions::varint())?)
            } else {
                None
            },
            value: match flags & 0x03 {
                0 => CommandNodeValue::Root,
                1 => CommandNodeValue::Literal {
                    name: String::read(&mut reader, StringOptions::default())?,
                },
                2 => CommandNodeValue::Argument {
                    name: String::read(&mut reader, StringOptions::default())?,
                    parser: ArgumentParser::read(&mut reader, ())?,
                    suggestions: if flags & 0x10 == 0x10 {
                        Some(SuggestionKind::read(&mut reader, ())?)
                    } else {
                        None
                    },
                },
                3 => return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "unsupported command type: 3",
                )),
                _ => unreachable!("0x03 only contains 0, 1, 2, 3")
            },
        })
    }
}

impl McWrite for CommandNode {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
        let mut flags = 0u8;
        match &self.value {
            CommandNodeValue::Root => flags |= 0x00,
            CommandNodeValue::Literal { .. } => flags |= 0x01,
            CommandNodeValue::Argument { suggestions, .. } => {
                flags |= 0x02;
                if suggestions.is_some() { flags |= 0x10; }
            }
        }
        if self.executable { flags |= 0x04; }
        if self.redirect.is_some() { flags |= 0x08; }

        flags.write(&mut writer, ())?;
        self.children.write(&mut writer, ListOptions::varint(IntOptions::varint()))?;
        if let Some(redirect) = self.redirect { redirect.write(&mut writer, IntOptions::varint())?; }
        match &self.value {
            CommandNodeValue::Root => {}
            CommandNodeValue::Literal { name } => {
                name.write(&mut writer, StringOptions::default())?;
            }
            CommandNodeValue::Argument { name, parser, suggestions } => {
                name.write(&mut writer, StringOptions::default())?;
                parser.write(&mut writer, ())?;
                if let Some(suggestions) = suggestions { suggestions.write(&mut writer, ())?; }
            }
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum CommandNodeValue {
    Root,
    Literal {
        name: String,
    },
    Argument {
        name: String,
        parser: ArgumentParser,
        suggestions: Option<SuggestionKind>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum ArgumentParser {
    Bool,
    Double {
        min: Option<f64>,
        max: Option<f64>,
    },
    Float {
        min: Option<f32>,
        max: Option<f32>,
    },
    Integer {
        min: Option<i32>,
        max: Option<i32>,
    },
    Long {
        min: Option<i64>,
        max: Option<i64>,
    },
    String(StringArgumentKind),
}

impl ArgumentParser {
    const KEY_BOOL: KeyRef<'static> = unsafe { KeyRef::new_unchecked("brigadier", "bool") };
    const KEY_DOUBLE: KeyRef<'static> = unsafe { KeyRef::new_unchecked("brigadier", "double") };
    const KEY_FLOAT: KeyRef<'static> = unsafe { KeyRef::new_unchecked("brigadier", "float") };
    const KEY_INTEGER: KeyRef<'static> = unsafe { KeyRef::new_unchecked("brigadier", "integer") };
    const KEY_LONG: KeyRef<'static> = unsafe { KeyRef::new_unchecked("brigadier", "long") };
    const KEY_STRING: KeyRef<'static> = unsafe { KeyRef::new_unchecked("brigadier", "string") };
}

impl McRead for ArgumentParser {
    type Options = ();

    fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
        let key = Key::read(&mut reader, KeyOptions::default())?;
        match key.as_ref() {
            Self::KEY_BOOL => Ok(Self::Bool),
            Self::KEY_DOUBLE => {
                let flags = reader.read_u8()?;
                Ok(Self::Double {
                    min: if flags & 0x01 == 0x01 { Some(reader.read_f64()?) } else { None },
                    max: if flags & 0x02 == 0x02 { Some(reader.read_f64()?) } else { None },
                })
            }
            Self::KEY_FLOAT => {
                let flags = reader.read_u8()?;
                Ok(Self::Float {
                    min: if flags & 0x01 == 0x01 { Some(reader.read_f32()?) } else { None },
                    max: if flags & 0x02 == 0x02 { Some(reader.read_f32()?) } else { None },
                })
            }
            Self::KEY_INTEGER => {
                let flags = reader.read_u8()?;
                Ok(Self::Integer {
                    min: if flags & 0x01 == 0x01 { Some(reader.read_i32()?) } else { None },
                    max: if flags & 0x02 == 0x02 { Some(reader.read_i32()?) } else { None },
                })
            }
            Self::KEY_LONG => {
                let flags = reader.read_u8()?;
                Ok(Self::Long {
                    min: if flags & 0x01 == 0x01 { Some(reader.read_i64()?) } else { None },
                    max: if flags & 0x02 == 0x02 { Some(reader.read_i64()?) } else { None },
                })
            }
            Self::KEY_STRING => Ok(Self::String(StringArgumentKind::read(reader, ())?)),
            v => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported argument type: {}", v),
            )),
        }
    }
}

impl McWrite for ArgumentParser {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
        match self {
            ArgumentParser::Bool => {
                Self::KEY_BOOL.write(&mut writer, KeyOptions::default())?;
            }
            ArgumentParser::Double { min, max } => {
                Self::KEY_DOUBLE.write(&mut writer, KeyOptions::default())?;
                let mut flags = 0u8;
                if min.is_some() { flags |= 0x01 }
                if max.is_some() { flags |= 0x02 }
                flags.write(&mut writer, ())?;
                if let Some(min) = min { min.write(&mut writer, ())?; }
                if let Some(max) = max { max.write(&mut writer, ())?; }
            }
            ArgumentParser::Float { min, max } => {
                Self::KEY_FLOAT.write(&mut writer, KeyOptions::default())?;
                let mut flags = 0u8;
                if min.is_some() { flags |= 0x01 }
                if max.is_some() { flags |= 0x02 }
                flags.write(&mut writer, ())?;
                if let Some(min) = min { min.write(&mut writer, ())?; }
                if let Some(max) = max { max.write(&mut writer, ())?; }
            }
            ArgumentParser::Integer { min, max } => {
                Self::KEY_INTEGER.write(&mut writer, KeyOptions::default())?;
                let mut flags = 0u8;
                if min.is_some() { flags |= 0x01 }
                if max.is_some() { flags |= 0x02 }
                flags.write(&mut writer, ())?;
                if let Some(min) = min { min.write(&mut writer, IntOptions::normal())?; }
                if let Some(max) = max { max.write(&mut writer, IntOptions::normal())?; }
            }
            ArgumentParser::Long { min, max } => {
                Self::KEY_LONG.write(&mut writer, KeyOptions::default())?;
                let mut flags = 0u8;
                if min.is_some() { flags |= 0x01 }
                if max.is_some() { flags |= 0x02 }
                flags.write(&mut writer, ())?;
                if let Some(min) = min { min.write(&mut writer, IntOptions::normal())?; }
                if let Some(max) = max { max.write(&mut writer, IntOptions::normal())?; }
            }
            ArgumentParser::String(kind) => {
                Self::KEY_STRING.write(&mut writer, KeyOptions::default())?;
                kind.write(&mut writer, ())?;
            }
        }
        Ok(())
    }
}

minecrevy_io_str::varint_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub enum StringArgumentKind {
        Word = 0,
        Quotable = 1,
        Greedy = 2,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SuggestionKind {
    AskServer,
    AllRecipes,
    AvailableSounds,
    AvailableBiomes,
    SummonableEntities,
}

impl SuggestionKind {
    const KEY_ASK_SERVER: KeyRef<'static> = unsafe { KeyRef::new_unchecked("minecraft", "ask_server") };
    const KEY_ALL_RECIPES: KeyRef<'static> = unsafe { KeyRef::new_unchecked("minecraft", "all_recipes") };
    const KEY_AVAILABLE_SOUNDS: KeyRef<'static> = unsafe { KeyRef::new_unchecked("minecraft", "available_sounds") };
    const KEY_AVAILABLE_BIOMES: KeyRef<'static> = unsafe { KeyRef::new_unchecked("minecraft", "available_biomes") };
    const KEY_SUMMONABLE_ENTITIES: KeyRef<'static> = unsafe { KeyRef::new_unchecked("minecraft", "summonable_entities 	") };
}

impl McRead for SuggestionKind {
    type Options = ();

    fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
        let key = Key::read(&mut reader, KeyOptions::default())?;
        match key.as_ref() {
            Self::KEY_ASK_SERVER => Ok(Self::AskServer),
            Self::KEY_ALL_RECIPES => Ok(Self::AllRecipes),
            Self::KEY_AVAILABLE_SOUNDS => Ok(Self::AvailableSounds),
            Self::KEY_AVAILABLE_BIOMES => Ok(Self::AvailableBiomes),
            Self::KEY_SUMMONABLE_ENTITIES => Ok(Self::SummonableEntities),
            v => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported suggestion type: {}", v),
            )),
        }
    }
}

impl McWrite for SuggestionKind {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
        match self {
            SuggestionKind::AskServer => Self::KEY_ASK_SERVER.write(&mut writer, KeyOptions::default())?,
            SuggestionKind::AllRecipes => Self::KEY_ALL_RECIPES.write(&mut writer, KeyOptions::default())?,
            SuggestionKind::AvailableSounds => Self::KEY_AVAILABLE_SOUNDS.write(&mut writer, KeyOptions::default())?,
            SuggestionKind::AvailableBiomes => Self::KEY_AVAILABLE_BIOMES.write(&mut writer, KeyOptions::default())?,
            SuggestionKind::SummonableEntities => Self::KEY_SUMMONABLE_ENTITIES.write(&mut writer, KeyOptions::default())?,
        }
        Ok(())
    }
}
