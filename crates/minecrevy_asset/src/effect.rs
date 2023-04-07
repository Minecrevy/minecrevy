use bevy::{
    prelude::{Handle, Resource},
    reflect::TypeUuid,
};
use derive_more::AsRef;
use minecrevy_core::key::Key;
use minecrevy_text::TextColor;

// TODO
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct EffectValue {
    /// The kind of status effect.
    pub effect: Handle<Effect>,
    /// How long the status effect lasts.
    pub duration: i32,
    /// Howe strong the status effect is.
    pub amplifier: i32,
    /// Whether the status effect's particles are semi-transparent.
    pub ambient: bool,
    /// Whether the status effect has visible particles.
    pub visible: bool,
    /// Whether the icon of the status effect is shown.
    pub show_icon: bool,
}

// TODO
#[derive(TypeUuid, AsRef, Clone, PartialEq, Debug)]
#[uuid = "76724ff0-4d6f-4212-9c4f-b54bcca7e7b7"]
pub struct Effect {
    #[as_ref]
    pub key: Key,
    pub category: Handle<EffectCategory>,
}

// TODO
#[derive(Resource)]
pub struct EffectCategories {
    pub beneficial: Handle<EffectCategory>,
    pub harmful: Handle<EffectCategory>,
    pub neutral: Handle<EffectCategory>,
}

// TODO
#[derive(TypeUuid, Clone, PartialEq, Debug)]
#[uuid = "e1e838a9-bd6e-4f4f-b8bd-be492ab4a784"]
pub struct EffectCategory {
    pub name: String,
    pub color: TextColor,
}
