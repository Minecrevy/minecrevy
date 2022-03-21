use std::collections::HashMap;

use minecrevy_io_str::{McRead, McWrite};
use minecrevy_key::Key;
use minecrevy_text::Text;

use crate::types::Slot;

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct Advancement {
    pub parent: Option<Key>,
    pub display: Option<AdvancementDisplay>, // TODO finish display field
    // pub criterion: Vec<AdvancementCriteria>,
    // pub requirements: Vec<AdvancementRequirement>,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AdvancementDisplay {
    pub title: Text,
    pub description: Text,
    pub icon: Slot,
    pub frame_type: AdvancementFrameKind,
    pub flags: i32,
    // pub background: Option<Key>, // TODO weird semantics
    // pub x: f32,
    // pub y: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, McRead, McWrite)]
#[io_repr(varint)]
pub enum AdvancementFrameKind {
    Task = 0,
    Challenge = 1,
    Goal = 2,
}

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AdvancementCriteria(pub Text);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AdvancementRequirement(pub Vec<String>);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct AdvancementProgress(pub HashMap<Key, CriteriaProgress>);

#[derive(Clone, PartialEq, Debug, McRead, McWrite)]
pub struct CriteriaProgress(pub Option<i64>);
