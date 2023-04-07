use std::{borrow::Cow, hash::Hash};

use minecrevy_core::key::Key;

pub use self::{color::*, style::*};

mod color;
mod compact;
mod style;

/// A text component.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub content: TextContent,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub style: TextStyle,
    // TODO: Vec<Text> --> Cow<'static, [Text]> after rustc bug fixed
    #[cfg_attr(feature = "serde", serde(rename = "extra"))]
    pub children: Vec<Text>,
}

impl Text {
    /// An empty text component.
    pub const EMPTY: Self = Self::str("");
    /// A text component with a `\n`.
    pub const NEWLINE: Self = Self::str("\n");
    /// A text component with a space.
    pub const SPACE: Self = Self::str(" ");

    /// Constructs a simple text component from the given string.
    pub const fn str(s: &'static str) -> Self {
        Self {
            style: TextStyle::empty(),
            content: TextContent::str(s),
            children: Vec::new(),
        }
    }

    /// Constructs a simple text component from the given string.
    pub fn string(s: impl Into<Cow<'static, str>>) -> Self {
        Self {
            style: TextStyle::empty(),
            content: TextContent::string(s),
            children: Vec::new(),
        }
    }

    pub fn style(&self) -> &TextStyle {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut TextStyle {
        &mut self.style
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn font(&self) -> Option<&Key> {
        self.style.font.as_ref()
    }

    pub fn font_mut(&mut self) -> Option<&mut Key> {
        self.style.font.as_mut()
    }

    pub fn with_font(mut self, font: impl Into<Option<Key>>) -> Self {
        self.style.font = font.into();
        self
    }

    pub fn color(&self) -> Option<&TextColor> {
        self.style.color.as_ref()
    }

    pub fn color_mut(&mut self) -> Option<&mut TextColor> {
        self.style.color.as_mut()
    }

    pub fn with_color(mut self, color: impl Into<Option<TextColor>>) -> Self {
        self.style.color = color.into();
        self
    }

    pub fn with_color_if_absent(mut self, color: impl Into<Option<TextColor>>) -> Self {
        if let None = self.style.color {
            self.style.color = color.into();
        }
        self
    }

    pub fn children(&self) -> &[Text] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Text> {
        &mut self.children
    }

    pub fn with_children(mut self, children: impl Into<Vec<Text>>) -> Self {
        self.children = children.into();
        self
    }

    pub fn push(&mut self, child: Text) {
        self.children.push(child);
    }

    pub fn compact(&mut self) {
        compact::compact(self, None);
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// The content of a [`Text`] component.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TextContent {
    /// A simple string.
    String(#[cfg_attr(feature = "serde", serde(rename = "text"))] Cow<'static, str>),
    /// A translation key, with optional arguments.
    Translation(
        #[cfg_attr(feature = "serde", serde(rename = "translate"))] Cow<'static, str>,
        #[cfg_attr(feature = "serde", serde(rename = "with"))] Vec<Text>,
    ),
    /// A keybind key.
    Keybind(#[cfg_attr(feature = "serde", serde(rename = "keybind"))] Cow<'static, str>),
    /// A scoreboard score.
    Score {
        name: Cow<'static, str>,
        objective: Cow<'static, str>,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "Option::is_none")
        )]
        value: Option<Cow<'static, str>>,
    },
}

impl TextContent {
    pub const fn str(s: &'static str) -> Self {
        Self::String(Cow::Borrowed(s))
    }

    pub fn string(s: impl Into<Cow<'static, str>>) -> Self {
        Self::String(s.into())
    }

    pub fn translation(
        translate: impl Into<Cow<'static, str>>,
        with: impl Into<Vec<Text>>,
    ) -> Self {
        Self::Translation(translate.into(), with.into())
    }

    pub fn keybind(keybind: impl Into<Cow<'static, str>>) -> Self {
        Self::Keybind(keybind.into())
    }

    pub fn score(
        name: impl Into<Cow<'static, str>>,
        objective: impl Into<Cow<'static, str>>,
        value: impl Into<Option<Cow<'static, str>>>,
    ) -> Self {
        Self::Score {
            name: name.into(),
            objective: objective.into(),
            value: value.into(),
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(str) => Some(&**str),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Text, TextStyle};

    #[test]
    fn constants() {
        assert_eq!(Text::EMPTY, Text::str(""));
        assert_eq!(Text::NEWLINE, Text::str("\n"));
        assert_eq!(Text::SPACE, Text::str(" "));
    }

    #[test]
    fn string() {
        let borrowed = Text::str("foo");

        assert_eq!(Some("foo"), borrowed.content.as_string());
        assert_eq!(TextStyle::empty(), borrowed.style);

        let owned = Text::string("bar");

        assert_eq!(Some("bar"), owned.content.as_string());
        assert_eq!(TextStyle::empty(), owned.style);
    }
}
