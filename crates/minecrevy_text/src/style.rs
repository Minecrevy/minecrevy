use std::borrow::Cow;

use enumflags2::{bitflags, BitFlags};
use minecrevy_core::key::Key;

use crate::{Text, TextColor};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyle {
    /// Whether text appears bold.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub bold: Option<bool>,
    /// Whether text appears in italics.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub italic: Option<bool>,
    /// Whether text has an underline.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub underlined: Option<bool>,
    /// Whether text has a strike through it.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub strikethrough: Option<bool>,
    /// Whether text is obfuscated/unreadable.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub obfuscated: Option<bool>,
    /// The font used for text.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub font: Option<Key>,
    /// The color of the text.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub color: Option<TextColor>,
    /// The text that is inserted when shift-clicked.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub insertion: Option<Cow<'static, str>>,
    /// The event that occurs when the text is clicked.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none", rename = "clickEvent")
    )]
    pub click: Option<ClickEvent>,
    /// The event that occurs when the text is hovered over.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none", rename = "hoverEvent")
    )]
    pub hover: Option<HoverEvent>,
}

impl TextStyle {
    /// Constructs an empty text style.
    pub const fn empty() -> Self {
        Self {
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            font: None,
            color: None,
            insertion: None,
            click: None,
            hover: None,
        }
    }

    /// Constructs a style with just the specified [`TextColor`].
    pub const fn from_color(color: TextColor) -> Self {
        Self {
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            font: None,
            color: Some(color),
            insertion: None,
            click: None,
            hover: None,
        }
    }

    /// Returns true if the style is empty.
    pub fn is_empty(&self) -> bool {
        *self == TextStyle::empty()
    }

    pub fn with_font(mut self, font: impl Into<Option<Key>>) -> Self {
        self.font = font.into();
        self
    }

    pub fn with_color(mut self, color: impl Into<Option<TextColor>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn with_color_if_absent(mut self, color: impl Into<Option<TextColor>>) -> Self {
        if let None = self.color {
            self.color = color.into();
        }
        self
    }

    pub fn merge(
        mut self,
        other: Self,
        strategy: MergeStrategy,
        kinds: BitFlags<MergeKind>,
    ) -> Self {
        if strategy == MergeStrategy::Never || other.is_empty() || kinds.is_empty() {
            return self;
        }
        if self.is_empty() && kinds.is_all() {
            return other;
        }

        if kinds.contains(MergeKind::Color) {
            if let Some(color) = other.color {
                if strategy == MergeStrategy::Always
                    || (strategy == MergeStrategy::IfAbsentOnTarget && self.color.is_none())
                {
                    self.color = Some(color);
                }
            }
        }

        todo!()
    }

    pub fn unmerge(&mut self, other: Self) {
        todo!()
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<TextColor> for TextStyle {
    fn from(color: TextColor) -> Self {
        Self::from_color(color)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "snake_case", tag = "action", content = "value")
)]
pub enum ClickEvent {
    OpenUrl(Cow<'static, str>),
    RunCommand(Cow<'static, str>),
    SuggestCommand(Cow<'static, str>),
    ChangePage(i32),
    CopyToClipboard(Cow<'static, str>),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "snake_case", tag = "action", content = "value")
)]
pub enum HoverEvent {
    ShowText(Box<Text>),
    // TODO: ShowItem
    // TODO: ShowEntity
}

impl From<Text> for HoverEvent {
    fn from(text: Text) -> Self {
        Self::ShowText(Box::new(text))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MergeStrategy {
    Always,
    Never,
    IfAbsentOnTarget,
}

#[bitflags]
#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MergeKind {
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
    Font,
    Color,
    Insertion,
    Events,
}

impl MergeKind {
    pub fn decorations() -> BitFlags<Self> {
        Self::Bold | Self::Italic | Self::Underlined | Self::Strikethrough | Self::Obfuscated
    }
}

#[cfg(test)]
mod tests {
    use crate::{TextColor, TextStyle};

    #[test]
    fn empty() {
        let style = TextStyle::empty();

        assert_eq!(None, style.bold);
        assert_eq!(None, style.italic);
        assert_eq!(None, style.underlined);
        assert_eq!(None, style.strikethrough);
        assert_eq!(None, style.obfuscated);
        assert_eq!(None, style.font);
        assert_eq!(None, style.color);
        assert_eq!(None, style.insertion);
        assert_eq!(None, style.click);
        assert_eq!(None, style.hover);
    }

    #[test]
    fn color() {
        assert_eq!(
            Some(TextColor::GREEN),
            TextStyle::from_color(TextColor::GREEN).color
        );
    }
}
