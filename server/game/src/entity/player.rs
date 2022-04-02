//! Components and systems for Minecraft player entities.

use std::time::Duration;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use flexstr::SharedStr;
use uuid::Uuid;

use minecrevy_text::{TextPosition, ChatVisibility, Title, Style};
use minecrevy_util::{GameMode, MainHand};

use crate::entity::living::*;
use crate::entity::*;

pub mod conn;
pub mod play;
pub mod tablist;
pub mod text;

/// [`Plugin`]s providing functionality for [`players`](PlayerBundle).
pub struct PlayerPlugins;

impl PluginGroup for PlayerPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(conn::ConnectionPlugin)
            .add(play::PlayPlugin)
            .add(text::TextPlugin)
            .add(tablist::TabListPlugin);
    }
}

/// Component [`Bundle`] for Minecraft player entities.
///
/// # Filtering
///
/// The easiest way to filter for only player entities is to use `With<Socket>`.
#[derive(Bundle, Clone, Debug, Default)]
pub struct PlayerBundle {
    /// See [`LivingEntityBundle`] for documentation.
    #[bundle]
    pub living: LivingEntityBundle,
    /// See [`KeepAlive`] for documentation.
    pub keepalive: KeepAlive,
    /// See [`SpawnPosition`] for documentation.
    pub position: SpawnPosition,
    /// See [`VisibleEntities`] for documentation.
    pub visible: VisibleEntities,
    /// See [`GameMode`] for documentation.
    pub gamemode: GameMode,
    /// See [`AbsorptionHearts`] for documentation.
    pub absorption: AbsorptionHearts,
    /// See [`Kills`] for documentation.
    pub kills: Kills,
    /// See [`DisplayedSkinParts`] for documentation.
    pub skin_parts: DisplayedSkinParts,
    /// See [`MainHand`] for documentation.
    pub main_hand: MainHand,
    /// See [`ShoulderEntities`] for documentation.
    pub shoulder: ShoulderEntities,
    /// See [`Titles`] for documentation.
    pub titles: Titles,
    /// See [`Messages`] for documentation.
    pub messages: Messages,
    /// See [`TabList`] for documentation.
    pub tablist: TabList,
}

/// The player's keep-alive connection status.
#[derive(Component, Clone, Debug)]
pub struct KeepAlive {
    /// The timer used to ensure clients respond to keep-alive packets in a timely manner.
    pub timer: Timer,
    /// The random ID used for client acknowledgement.
    /// `None` if not waiting for a client acknowledgement.
    pub id: Option<i64>,
}

impl KeepAlive {
    /// The number of seconds per keep-alive sent.
    pub const EVERY_X_SECONDS: u64 = 15;

    pub const FAILED: Text = Text::str("Failed to respond to Keep-Alive packet", Style::empty());
}

impl Default for KeepAlive {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(Self::EVERY_X_SECONDS), true),
            id: None,
        }
    }
}

/// The spawn position of the player (i.e. where the compass points).
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct SpawnPosition {
    /// The player's spawn position.
    pub position: Vector3<i32>,
    /// The player's spawn pitch angle.
    pub pitch: f32,
}

/// Which entities are visible to the player.
/// A visible entity will have packets about them sent to the player.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct VisibleEntities(pub Vec<NetworkId>);

/// How many additional hearts the player has, through the absorption potion effect.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct AbsorptionHearts(pub f32);

/// How many other entities the player has killed.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Kills(pub i32);

/// Which optional parts of the player's skin are displayed.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct DisplayedSkinParts {
    /// True if the cape is shown.
    pub cape: bool,
    /// True if the jacket (torso overlay) is shown.
    pub jacket: bool,
    /// True if the left sleeve (left arm overlay) is shown.
    pub left_sleeve: bool,
    /// True if the right sleeve (right arm overlay) is shown.
    pub right_sleeve: bool,
    /// True if the left pants (left leg overlay) is shown.
    pub left_pants: bool,
    /// True if the right pants (right leg overlay) is shown.
    pub right_pants: bool,
    /// True if the hat (head overlay) is shown.
    pub hat: bool,
}

/// The entity data for entities sitting on the player's shoulders.
/// Exclusively used for parrots, currently.
#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct ShoulderEntities {
    /// The entity on the player's left shoulder.
    pub left: Option<nbt::Blob>,
    /// The entity on the player's right shoulder.
    pub right: Option<nbt::Blob>,
}

/// A buffer of [`Title`]s to be sent to the player.
#[derive(Component, Clone, Debug, Default)]
pub struct Titles {
    titles: Vec<Title>,
    clear: bool,
}

impl Titles {
    /// Clears the buffer and appends a [`Title`].
    #[inline]
    pub fn set(&mut self, title: Title) {
        self.titles.clear();
        self.titles.push(title);
    }

    /// Appends a [`Title`] to the end of the buffer.
    #[inline]
    pub fn push(&mut self, title: Title) {
        self.titles.push(title);
    }

    /// Extends the buffer with a collection of [`Title`]s.
    #[inline]
    pub fn extend(&mut self, titles: impl IntoIterator<Item=Title>) {
        self.titles.extend(titles)
    }

    /// Removes all [`Title`]s from the buffer,
    /// and flags for a clear packet to be sent to the player.
    #[inline]
    pub fn clear(&mut self) {
        self.titles.clear();
        self.clear = true;
    }

    /// Drains all [`Title`]s from the buffer.
    #[inline]
    pub fn drain(&mut self) -> impl Iterator<Item = Title> + '_ {
        self.titles.drain(..)
    }
}

/// A buffer of [`Message`]s to be sent to the player.
#[derive(Component, Clone, Debug, Default)]
pub struct Messages(Vec<Message>);

impl Messages {
    /// Appends a [`Message`] to the end of the buffer.
    #[inline]
    pub fn push(&mut self, message: Message) {
        self.0.push(message);
    }

    /// Extends the buffer with a collection of [`Message`]s.
    #[inline]
    pub fn extend(&mut self, messages: impl IntoIterator<Item=Message>) {
        self.0.extend(messages)
    }

    /// Removes all [`Message`]s from the buffer.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Drains all [`Message`]s from the buffer.
    #[inline]
    pub fn drain(&mut self) -> impl Iterator<Item = Message> + '_ {
        self.0.drain(..)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Message {
    /// The message content.
    pub message: Text,
    /// The message [`position`](TextPosition).
    pub position: TextPosition,
    /// The [`Uuid`] of the message sender.
    pub sender: Uuid,
}

impl Message {
    pub fn chat(message: Text) -> Self {
        Self {
            message,
            position: TextPosition::Chat,
            sender: Uuid::nil(),
        }
    }

    pub fn chat_from(message: Text, sender: Uuid) -> Self {
        Self {
            message,
            position: TextPosition::Chat,
            sender,
        }
    }

    pub fn action_bar(message: Text) -> Self {
        Self {
            message,
            position: TextPosition::ActionBar,
            sender: Uuid::nil(),
        }
    }

    pub fn system(message: Text) -> Self {
        Self {
            message,
            position: TextPosition::System,
            sender: Uuid::nil(),
        }
    }
}

/// The player's tab-list (`<tab>`) header and footer.
#[derive(Component, Clone, Debug, Default)]
pub struct TabList {
    /// The tab-list header.
    pub header: Option<Text>,
    /// The tab-list footer.
    pub footer: Option<Text>,
}

#[derive(Component, Clone, Debug)]
pub struct ClientSettings {
    pub locale: SharedStr,
    pub view_dst: i8,
    pub visibility: ChatVisibility,
    pub colors: bool,
    // TODO: strongly typed bit mask
    pub skin_parts: u8,
    pub main_hand: MainHand,
    pub filter_text: bool,
    pub shown_on_tablist: bool,
}
