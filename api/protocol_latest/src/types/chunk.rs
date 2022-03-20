use std::io::{self, Read, Write};

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};
use minecrevy_io_str::{BitSet, McRead, McWrite};

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ChunkData {
    pub heightmaps: nbt::Value,
    pub sections: Vec<ChunkSection>,
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ChunkSection {
    /// The number of non-air blocks present in the chunk section.
    pub num_blocks: i16,
    pub block_states: PalettedContainer,
    pub biomes: PalettedContainer,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct PalettedContainer {
    pub palette: Palette,
    pub entry_ids: Vec<i64>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Palette {
    SingleValue(SingleValuePalette),
    Indirect { bits_per_entry: u8, palette: IndirectPalette },
    Direct { bits_per_entry: u8 },
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SingleValuePalette {
    #[options(varint = true)]
    pub value: i32,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct IndirectPalette {
    #[options(inner.varint = true)]
    pub values: Vec<i32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct PaletteOptions {
    pub kind: Option<PaletteKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PaletteKind {
    BlockStates,
    Biomes,
}

impl From<&str> for PaletteKind {
    fn from(v: &str) -> Self {
        match v {
            "block_states" => PaletteKind::BlockStates,
            "biomes" => PaletteKind::Biomes,
            v => panic!("invalid palette kind: {}", v),
        }
    }
}

impl McRead for Palette {
    type Options = PaletteOptions;

    fn read<R: Read>(mut reader: R, options: Self::Options) -> io::Result<Self> {
        let kind = options.kind.expect("must specify palette kind");

        let bits_per_entry = reader.read_u8()?;
        match (kind, bits_per_entry) {
            (_, 0) => Ok(Palette::SingleValue(SingleValuePalette::read(reader, ())?)),
            (PaletteKind::BlockStates, 1..=8) | (PaletteKind::Biomes, 1..=3) => Ok(Palette::Indirect {
                bits_per_entry,
                palette: IndirectPalette::read(reader, ())?,
            }),
            (_, _) => Ok(Palette::Direct {
                bits_per_entry
            }),
        }
    }
}

impl McWrite for Palette {
    type Options = PaletteOptions;

    fn write<W: Write>(&self, mut writer: W, _: Self::Options) -> io::Result<()> {
        match self {
            Palette::SingleValue(palette) => {
                writer.write_u8(0)?;
                palette.write(writer, ())?;
            }
            Palette::Indirect { bits_per_entry, palette } => {
                writer.write_u8(*bits_per_entry)?;
                palette.write(writer, ())?;
            }
            Palette::Direct { bits_per_entry } => {
                writer.write_u8(*bits_per_entry)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BlockEntity {
    pub section_xz: u8,
    pub y: i16,
    #[options(varint = true)]
    pub kind: i32,
    pub data: nbt::Value,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct LightData {
    pub trust_edges: bool,
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light: Vec<Vec<u8>>,
    pub block_light: Vec<Vec<u8>>,
}
