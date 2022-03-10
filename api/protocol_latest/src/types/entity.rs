use std::collections::HashMap;
use std::io::{Read, Write};

use glam::{IVec3, Vec3};
use uuid::Uuid;

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};
use minecrevy_io_str::{McRead, McWrite};
use minecrevy_text::Text;

use crate::types::{Direction, Slot};

#[derive(Clone, PartialEq, Debug)]
pub struct Metadata(pub HashMap<u8, MetadataValue>);

impl McRead for Metadata {
    type Options = ();

    fn read<R: Read>(mut reader: R, (): Self::Options) -> std::io::Result<Self> {
        let mut result = HashMap::new();
        loop {
            let idx = match reader.read_u8()? {
                0xFF => break,
                idx => idx,
            };
            result.insert(idx, MetadataValue::read(&mut reader, ())?);
        }
        Ok(Metadata(result))
    }
}

impl McWrite for Metadata {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> std::io::Result<()> {
        for (&idx, value) in &self.0 {
            writer.write_u8(idx)?;
            value.write(&mut writer, ())?;
        }
        writer.write_u8(0xFF)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
#[mcio(kind = "varint")]
pub enum MetadataValue {
    /// type: 0
    Byte(i8),
    VarInt(#[mcio(varint)] i32),
    Float(f32),
    String(String),
    Text(Text),
    OptText(Option<Text>),
    Slot(Slot),
    Bool(bool),
    Rotation(Vec3),
    Position(#[mcio(compressed)] IVec3),
    OptPosition(#[mcio(inner::compressed)] Option<IVec3>),
    Direction(Direction),
    OptUuid(Option<Uuid>),
    OptBlockId(#[mcio(varint)] i32),
    Nbt(nbt::Blob),
    // TODO: https://wiki.vg/Entity_metadata#Entity_Metadata_Format
    Particle(()),
    Villager(Villager),
    OptVarInt(#[mcio(varint)] i32),
    /// type: 18
    Pose(Pose),
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Villager {
    pub kind: VillagerKind,
    pub profession: VillagerProfession,
    #[mcio(varint)]
    pub level: i32,
}

minecrevy_io_str::varint_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub enum VillagerKind {
        Desert = 0,
        Jungle = 1,
        Plains = 2,
        Savanna = 3,
        Snow = 4,
        Swamp = 5,
        Taiga = 6,
    }
}

minecrevy_io_str::varint_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub enum VillagerProfession {
        None = 0,
        Armorer = 1,
        Butcher = 2,
        Cartographer = 3,
        Cleric = 4,
        Farmer = 5,
        Fisherman = 6,
        Fletcher = 7,
        Leatherworker = 8,
        Librarian = 9,
        Mason = 10,
        Nitwit = 11,
        Shepherd = 12,
        Toolsmith = 13,
        Weaponsmith = 14,
    }
}

minecrevy_io_str::varint_enum! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub enum Pose {
        Standing = 0,
        FallFlying = 1,
        Sleeping = 2,
        Swimming = 3,
        SpinAttack = 4,
        Crouching = 5,
        LongJumping = 6,
        Dying = 7,
    }
}
