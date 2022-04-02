//! Types used in the Minecraft protocol.

use std::error::Error;
use std::io::{self, Read, Write};
use std::marker::PhantomData;

use uuid::Uuid;

use minecrevy_io_buf::{ReadMinecraftExt, WriteMinecraftExt};
use minecrevy_io_str::{VectorOptions, IntOptions, McRead, McWrite};
use minecrevy_key::{key, Key, KeyOptions, StaticKey};
use minecrevy_math::num::{One, Zero};
use minecrevy_math::vector::Vector;
use minecrevy_text::Text;
use minecrevy_util::GameMode;

pub use self::advancement::*;
pub use self::chunk::*;
pub use self::cmd::*;
pub use self::entity::*;
pub use self::recipes::*;
pub use self::scoreboard::*;
pub use self::world::*;

mod advancement;
mod chunk;
mod cmd;
mod entity;
mod recipes;
mod scoreboard;
mod world;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct AsPrimitive<T, Proto>(pub T, PhantomData<Proto>);

impl<T, Proto> From<T> for AsPrimitive<T, Proto> {
    fn from(v: T) -> Self {
        Self(v, PhantomData)
    }
}

impl<T, Proto> McRead for AsPrimitive<T, Proto>
where
    T: TryFrom<Proto>,
    Proto: McRead,
    <T as TryFrom<Proto>>::Error: Error + Send + Sync + 'static,
{
    type Options = Proto::Options;

    fn read<R: Read>(reader: R, options: Self::Options) -> io::Result<Self> {
        let value = T::try_from(Proto::read(reader, options)?)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Self(value, PhantomData))
    }
}

impl<T, Proto> McWrite for AsPrimitive<T, Proto>
where
    T: Clone,
    Proto: TryFrom<T> + McWrite,
    <Proto as TryFrom<T>>::Error: Error + Send + Sync + 'static,
{
    type Options = Proto::Options;

    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()> {
        let value = Proto::try_from(self.0.clone())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        value.write(writer, options)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct BoolNum<T>(pub bool, PhantomData<T>);

impl<T> From<bool> for BoolNum<T> {
    #[inline]
    fn from(v: bool) -> Self {
        Self(v, PhantomData)
    }
}

impl<T> From<BoolNum<T>> for bool {
    fn from(v: BoolNum<T>) -> Self {
        v.0
    }
}

impl<T: Zero + PartialEq + McRead> McRead for BoolNum<T> {
    type Options = T::Options;

    fn read<R: Read>(reader: R, options: Self::Options) -> io::Result<Self> {
        let value = T::read(reader, options)?;
        Ok(Self::from(!value.is_zero()))
    }
}

impl<T: Zero + One + McWrite> McWrite for BoolNum<T> {
    type Options = T::Options;

    fn write<W: Write>(&self, writer: W, options: Self::Options) -> io::Result<()> {
        let value: T = match self.0 {
            false => T::ZERO,
            true => T::ONE,
        };

        value.write(writer, options)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct PreviousGameMode(pub Option<GameMode>);

impl McRead for PreviousGameMode {
    type Options = ();

    fn read<R: Read>(mut reader: R, _options: Self::Options) -> io::Result<Self> {
        Ok(match reader.read_i8()? {
            -1 => Self(None),
            0 => Self(Some(GameMode::Survival)),
            1 => Self(Some(GameMode::Creative)),
            2 => Self(Some(GameMode::Adventure)),
            3 => Self(Some(GameMode::Spectator)),
            v => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("expected previous gamemode -1, 0, 1, 2, or 3 but got {}", v),
                ));
            }
        })
    }
}

impl McWrite for PreviousGameMode {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, _options: Self::Options) -> io::Result<()> {
        match self.0 {
            None => writer.write_i8(-1)?,
            Some(GameMode::Survival) => writer.write_i8(0)?,
            Some(GameMode::Creative) => writer.write_i8(1)?,
            Some(GameMode::Adventure) => writer.write_i8(2)?,
            Some(GameMode::Spectator) => writer.write_i8(3)?,
        }
        Ok(())
    }
}

#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PlayerAbilities {
    Invulnerable = 0x01,
    Flying = 0x02,
    AllowFlying = 0x04,
    InstantBreak = 0x08,
}

/// An `Angle` as known in the Minecraft protocol is 1/256th of a turn, represented as a single byte.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default, McRead, McWrite)]
pub struct Angle(pub u8);

impl Angle {
    const RATIO: f32 = 256.0 / 360.0;
}

impl From<i8> for Angle {
    fn from(v: i8) -> Self {
        // We don't actually care about the signedness.
        // The resultant angle is the same.
        Angle(v as u8)
    }
}

impl From<f32> for Angle {
    #[inline]
    fn from(v: f32) -> Self {
        Angle((v / Angle::RATIO) as u8)
    }
}

impl From<Angle> for f32 {
    #[inline]
    fn from(Angle(v): Angle) -> Self {
        v as f32 / Angle::RATIO
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SignalDestination {
    Block(Vector<3, i32>),
    Entity(i32),
}

impl SignalDestination {
    pub const BLOCK: StaticKey = key!(ref "minecraft:block");
    pub const ENTITY: StaticKey = key!(ref "minecraft:entity");
}

impl McRead for SignalDestination {
    type Options = ();

    fn read<R: Read>(mut reader: R, _options: Self::Options) -> io::Result<Self> {
        let key = Key::read(&mut reader, KeyOptions::default())?;

        match key.as_ref() {
            Self::BLOCK => Ok(Self::Block(Vector::read(
                reader,
                VectorOptions { compressed: true },
            )?)),
            Self::ENTITY => Ok(Self::Entity(i32::read(reader, IntOptions::varint())?)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported destination type: {}", key),
            )),
        }
    }
}

impl McWrite for SignalDestination {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, _options: Self::Options) -> io::Result<()> {
        match self {
            Self::Block(pos) => {
                Self::BLOCK.write(&mut writer, KeyOptions::default())?;
                pos.write(writer, VectorOptions { compressed: true })?;
            }
            Self::Entity(id) => {
                Self::ENTITY.write(&mut writer, KeyOptions::default())?;
                id.write(writer, IntOptions::varint())?;
            }
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(u8)]
pub enum Animation {
    SwingMainArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    SwingOffHand = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
pub struct Statistic {
    /// The category ID.
    #[options(varint = true)]
    pub category: i32,
    /// The statistic ID.
    #[options(varint = true)]
    pub statistic: i32,
    /// The statistic's value.
    #[options(varint = true)]
    pub value: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum BossBarColor {
    Pink = 0,
    Blue = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Purple = 5,
    White = 6,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum BossBarStyle {
    Solid = 0,
    Notched6 = 1,
    Notched10 = 2,
    Notched12 = 3,
    Notched20 = 4,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct TabCompletionMatch {
    #[options(max_len = 32767)]
    pub text: String,
    pub tooltip: Option<Text>,
}

pub type Slot = Option<SlotData>;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct SlotData {
    #[options(varint = true)]
    pub item_id: i32,
    pub count: i8,
    pub data: nbt::Value,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct BlockOffset(pub [i8; 3]);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Icon {
    #[options(varint = true)]
    pub kind: i32,
    pub x: i8,
    pub z: i8,
    pub direction: i8,
    pub display_name: Option<Text>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Trade {
    pub input_1: Slot,
    pub output: Slot,
    pub input_2: Option<Slot>,
    pub disabled: bool,
    pub num_trade_uses: i32,
    pub max_trade_uses: i32,
    pub xp: i32,
    pub special_price: i32,
    pub price_mul: f32,
    pub demand: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum TabListActionKind {
    AddPlayer = 0,
    SetGameMode = 1,
    SetPing = 2,
    SetDisplayName = 3,
    RemovePlayer = 4,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
#[io_repr(varint)]
pub enum TabListActions {
    AddPlayer(Vec<(Uuid, AddPlayer)>),
    SetGameMode(Vec<(Uuid, i32)>),
    SetPing(Vec<(Uuid, i32)>),
    SetDisplayName(Vec<(Uuid, Option<Text>)>),
    RemovePlayer(Vec<Uuid>),
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AddPlayer {
    #[options(max_len = 16)]
    pub name: String,
    pub properties: Vec<ProfileProperty>,
    #[options(varint = true)]
    pub gamemode: i32,
    #[options(varint = true)]
    pub ping: i32,
    pub display_name: Option<Text>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityAttribute {
    pub key: Key,
    pub value: f64,
    pub modifiers: Vec<EntityAttributeModifier>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct EntityAttributeModifier {
    pub uuid: Uuid,
    pub amount: f64,
    pub operation: EntityAttributeModifierOperation,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(u8)]
pub enum EntityAttributeModifierOperation {
    AddAmount = 0,
    AddPercent = 1,
    MultiplyPercent = 2,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Tag {
    pub name: Key,
    #[options(inner.varint = true)]
    pub entries: Vec<i32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum ClientStatusAction {
    Respawn = 0,
    RequestStats = 1,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum EntityInteractionKind {
    Interact = 0,
    Attack = 1,
    InteractAt = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum BlockDigStatus {
    Started = 0,
    Cancelled = 1,
    Finished = 2,
    DropStack = 3,
    Drop = 4,
    UseItem = 5,
    SwapItem = 6,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(u8)]
pub enum Face {
    Bottom = 0,
    Top = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum EntityActionKind {
    SneakStart = 0,
    SneakStop = 1,
    BedLeave = 2,
    SprintStart = 3,
    SprintStop = 4,
    HorseJumpStart = 5,
    HorseJumpStop = 6,
    HorseInventoryOpen = 7,
    ElytraFlightStart = 8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum RecipeBookKind {
    Crafting = 0,
    Furnace = 1,
    BlastFurnace = 2,
    Smoker = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum ResourcePackResult {
    Loaded = 0,
    Declined = 1,
    Failed = 2,
    Accepted = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum CommandBlockMode {
    Sequence = 0,
    Auto = 1,
    Redstone = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum StructureBlockAction {
    UpdateData = 0,
    Save = 1,
    Load = 2,
    DetectSize = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum StructureBlockMode {
    Save = 0,
    Load = 1,
    Corner = 2,
    Data = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum StructureBlockMirror {
    None = 0,
    LeftRight = 1,
    FrontBack = 2,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum StructureBlockRotation {
    None = 0,
    Clockwise90 = 1,
    Clockwise180 = 2,
    CounterClockwise90 = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum FaceMode {
    Feet = 0,
    Eyes = 1,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct FaceEntity {
    #[options(varint = true)]
    pub entity_id: i32,
    pub entity_mode: FaceMode,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParticleData {
    Empty,
    BlockState(i32),
    Dust {
        rgb: Vector<3, f32>,
        scale: f32,
    },
    DustTransition {
        from_rgb: Vector<3, f32>,
        scale: f32,
        to_rgb: Vector<3, f32>,
    },
    Item(Slot),
    Vibration {
        origin: Vector<3, i32>,
        vibration: ParticleDataVibration,
        ticks: i32,
    },
}

impl McRead for ParticleData {
    type Options = ();

    fn read<R: Read>(_reader: R, (): Self::Options) -> io::Result<Self> {
        todo!()
    }
}

impl McWrite for ParticleData {
    type Options = ();

    fn write<W: Write>(&self, _writer: W, (): Self::Options) -> io::Result<()> {
        todo!()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParticleDataVibration {
    Block(Vector<3, i32>),
    Entity(i32),
}

impl McRead for ParticleDataVibration {
    type Options = ();

    fn read<R: Read>(mut reader: R, (): Self::Options) -> io::Result<Self> {
        let key = Key::read(&mut reader, KeyOptions::default())?;
        match key.as_ref() {
            SignalDestination::BLOCK => Ok(Self::Block(Vector::read(
                reader,
                VectorOptions { compressed: true },
            )?)),
            SignalDestination::ENTITY => Ok(Self::Entity(i32::read(reader, IntOptions::varint())?)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unsupported vibration type: {}", key),
            )),
        }
    }
}

impl McWrite for ParticleDataVibration {
    type Options = ();

    fn write<W: Write>(&self, mut writer: W, (): Self::Options) -> io::Result<()> {
        match self {
            Self::Block(pos) => {
                SignalDestination::BLOCK.write(&mut writer, KeyOptions::default())?;
                pos.write(writer, VectorOptions { compressed: true })?;
            }
            Self::Entity(id) => {
                SignalDestination::ENTITY.write(&mut writer, KeyOptions::default())?;
                id.write(writer, IntOptions::varint())?;
            }
        }
        Ok(())
    }
}
