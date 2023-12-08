//! A Minecraft text component library.

#![warn(missing_docs)]

use std::io::{self, Read, Write};

use minecrevy_io::{args::StringArgs, McRead, McWrite};
use serde::{Deserialize, Serialize};

pub mod prelude {
    //! Re-exports important traits and types.

    pub use super::{ClickEvent, HoverEvent, Text, TextContent, TextStyle};
}

/// A text component.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Text {
    /// The content of this text component.
    #[serde(flatten)]
    pub content: TextContent,
    /// The style of this text component.
    #[serde(flatten)]
    pub style: TextStyle,
    /// The children of this text component.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<Text>,
}

impl Text {
    /// Creates a new text component with the given [`String`] content.
    pub fn string(text: impl Into<String>) -> Self {
        Text {
            content: TextContent::string(text),
            style: TextStyle::default(),
            extra: Vec::new(),
        }
    }

    /// Creates a new text component with no content.
    pub fn empty() -> Self {
        Text::string("")
    }

    /// Creates a new text component with a space as content.
    pub fn space() -> Self {
        Text::string(" ")
    }

    /// Creates a new text component with a newline as content.
    pub fn newline() -> Self {
        Text::string("\n")
    }

    /// Sets [`TextStyle::bold`] to `true`.
    pub fn bold(mut self) -> Self {
        self.style.bold = Some(true);
        self
    }

    /// Sets [`TextStyle::italic`] to `true`.
    pub fn italic(mut self) -> Self {
        self.style.italic = Some(true);
        self
    }

    /// Sets [`TextStyle::underlined`] to `true`.
    pub fn underlined(mut self) -> Self {
        self.style.underlined = Some(true);
        self
    }

    /// Sets [`TextStyle::strikethrough`] to `true`.
    pub fn strikethrough(mut self) -> Self {
        self.style.strikethrough = Some(true);
        self
    }

    /// Sets [`TextStyle::obfuscated`] to `true`.
    pub fn obfuscated(mut self) -> Self {
        self.style.obfuscated = Some(true);
        self
    }

    /// Sets [`TextStyle::font`] to the given value.
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.style.font = Some(font.into());
        self
    }

    /// Sets [`TextStyle::insertion`] to the given value.
    pub fn insertion(mut self, insertion: impl Into<String>) -> Self {
        self.style.insertion = Some(insertion.into());
        self
    }

    /// Sets [`TextStyle::click`] to the given [`ClickEvent`].
    pub fn click(mut self, event: impl Into<ClickEvent>) -> Self {
        self.style.click = Some(event.into());
        self
    }

    /// Sets [`TextStyle::hover`] to the given [`HoverEvent`].
    pub fn hover(mut self, event: impl Into<HoverEvent>) -> Self {
        self.style.hover = Some(event.into());
        self
    }
}

impl From<String> for Text {
    fn from(content: String) -> Self {
        Text {
            content: TextContent::String { text: content },
            style: TextStyle::default(),
            extra: Vec::new(),
        }
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Self {
        Text::from(content.to_owned())
    }
}

/// Arguments for reading/writing a [`Text`] component.
#[derive(Clone, Debug)]
pub struct TextArgs {
    /// Specifies that the encoded/decoded text should not exceed the given length.
    ///
    /// Set to `None` to disable this limit.
    pub max_len: Option<usize>,
}

impl Default for TextArgs {
    fn default() -> Self {
        TextArgs {
            max_len: Some(262144),
        }
    }
}

impl McRead for Text {
    type Args = TextArgs;

    fn read(reader: impl Read, args: Self::Args) -> io::Result<Self> {
        let json = String::read(
            reader,
            StringArgs {
                max_len: args.max_len,
            },
        )?;

        serde_json::from_str::<Text>(&json)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl McWrite for Text {
    type Args = TextArgs;

    fn write(&self, writer: impl Write, args: Self::Args) -> io::Result<()> {
        let json = serde_json::to_string::<Text>(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        json.write(
            writer,
            StringArgs {
                max_len: args.max_len,
            },
        )
    }
}

/// The content of a text component.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextContent {
    /// A plain string.
    String {
        /// The literal text.
        text: String,
    },
    /// A translatable string.
    Translatable {
        /// The translation key.
        #[serde(rename = "translate")]
        key: String,
        /// The arguments to the translation.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        with: Vec<Text>,
    },
    /// A keybind.
    Keybind {
        /// The keybind code.
        keybind: String,
    },
}

impl TextContent {
    /// Creates a new text component with the given [`String`] content.
    pub fn string(text: impl Into<String>) -> Self {
        TextContent::String { text: text.into() }
    }

    /// Creates a new text component with the given translatable key.
    pub fn translatable(key: impl Into<String>) -> Self {
        TextContent::Translatable {
            key: key.into(),
            with: Vec::new(),
        }
    }

    /// Creates a new text component with the given keybind key.
    pub fn keybind(key: impl Into<String>) -> Self {
        TextContent::Keybind {
            keybind: key.into(),
        }
    }
}

/// The style of a text component.
#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
#[derive(Serialize, Deserialize)]
pub struct TextStyle {
    /// Whether this text component is bold.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// Whether this text component is italic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// Whether this text component is underlined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    /// Whether this text component is strikethrough.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    /// Whether this text component is obfuscated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    /// The font of this text component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// The color of this text component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The shift-click text of this text component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    /// The click event of this text component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click: Option<ClickEvent>,
    /// The hover event of this text component.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverEvent>,
}

/// Events that can be triggered by clicking on a text component.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
pub enum ClickEvent {
    /// Prompts the user to open the given URL.
    OpenUrl(String),
    /// Runs the given command as the user.
    RunCommand(String),
    /// Suggests the given command to the user.
    SuggestCommand(String),
    /// Changes the page of a book.
    ChangePage(i32),
    /// Prompts the user to copy the given text to their clipboard.
    CopyToClipboard(String),
}

impl ClickEvent {
    /// Creates a new [`ClickEvent::OpenUrl`] event.
    pub fn open_url(url: impl Into<String>) -> Self {
        ClickEvent::OpenUrl(url.into())
    }

    /// Creates a new [`ClickEvent::RunCommand`] event.
    pub fn run_command(command: impl Into<String>) -> Self {
        ClickEvent::RunCommand(command.into())
    }

    /// Creates a new [`ClickEvent::SuggestCommand`] event.
    pub fn suggest_command(command: impl Into<String>) -> Self {
        ClickEvent::SuggestCommand(command.into())
    }

    /// Creates a new [`ClickEvent::ChangePage`] event.
    pub fn change_page(page: i32) -> Self {
        ClickEvent::ChangePage(page)
    }

    /// Creates a new [`ClickEvent::CopyToClipboard`] event.
    pub fn copy_to_clipboard(text: impl Into<String>) -> Self {
        ClickEvent::CopyToClipboard(text.into())
    }
}

/// Events that can be triggered by hovering over a text component.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
pub enum HoverEvent {
    /// Shows the given text component to the user.
    ShowText(Box<Text>),
}

impl HoverEvent {
    /// Creates a new [`HoverEvent::ShowText`] event.
    pub fn show_text(text: impl Into<Text>) -> Self {
        HoverEvent::ShowText(Box::new(text.into()))
    }
}

impl From<Text> for HoverEvent {
    fn from(text: Text) -> Self {
        HoverEvent::show_text(text)
    }
}
