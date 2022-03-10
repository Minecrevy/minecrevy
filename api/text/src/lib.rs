use std::borrow::Cow;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::{de::Error, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use crate::bow::Bow;

#[cfg(all(feature = "serde_json", feature = "minecrevy_io_str"))]
pub use self::io_str::*;
#[cfg(feature = "serde_json")]
pub use self::json::*;

#[cfg(all(feature = "serde_json", feature = "minecrevy_io_str"))]
mod io_str;
#[cfg(feature = "serde_json")]
mod json;

pub mod bow;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub struct Text {
    #[serde(flatten)]
    pub value: TextValue,
    #[serde(flatten)]
    pub style: Style,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<Text>,
}

impl Text {
    pub const fn str(text: &'static str, style: Style) -> Self {
        Self {
            value: TextValue::String { text: Cow::Borrowed(text) },
            style,
            extra: vec![],
        }
    }

    pub const fn empty() -> Self {
        Self::str("", Style::empty())
    }

    pub const fn space() -> Self {
        Self::str(" ", Style::empty())
    }

    pub const fn newline() -> Self {
        Self::str("\n", Style::empty())
    }

    pub fn to_plain_string(&self) -> String {
        let mut result = String::new();
        self.plain_append_to(&mut result);
        result
    }

    fn plain_append_to(&self, out: &mut String) {
        if let TextValue::String { text } = &self.value {
            out.push_str(text);
        }
        for text in &self.extra {
            text.plain_append_to(out);
        }
    }

    pub fn with_child(mut self, child: Text) -> Self {
        self.extra.push(child);
        self
    }

    pub fn append_child(&mut self, child: Text) -> &mut Self {
        self.extra.push(child);
        self
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextValue {
    String {
        text: Cow<'static, str>
    },
    Translation {
        translate: Cow<'static, str>,
        with: Vec<Cow<'static, str>>,
    },
    Keybind {
        keybind: Cow<'static, str>
    },
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<Cow<'static, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insertion: Option<Cow<'static, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

impl Style {
    pub const fn empty() -> Self {
        Self {
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            font: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }
    }

    pub const fn color(color: Color) -> Self {
        Self {
            color: Some(color),
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            font: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum ClickEvent {
    OpenUrl(Cow<'static, str>),
    RunCommand(Cow<'static, str>),
    SuggestCommand(Cow<'static, str>),
    ChangePage(i32),
    CopyToClipboard(Cow<'static, str>),
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum HoverEvent {
    ShowText(Bow<'static, Text>),
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const DARK_BLUE: Color = Color::new(0, 0, 170);
    pub const DARK_GREEN: Color = Color::new(0, 170, 0);
    pub const DARK_CYAN: Color = Color::new(0, 170, 170);
    pub const DARK_RED: Color = Color::new(170, 0, 0);
    pub const DARK_PURPLE: Color = Color::new(170, 0, 170);
    pub const GOLD: Color = Color::new(255, 170, 0);
    pub const GRAY: Color = Color::new(170, 170, 170);
    pub const DARK_GRAY: Color = Color::new(85, 85, 85);
    pub const BLUE: Color = Color::new(85, 85, 255);
    pub const GREEN: Color = Color::new(85, 255, 85);
    pub const AQUA: Color = Color::new(85, 255, 255);
    pub const RED: Color = Color::new(255, 85, 85);
    pub const LIGHT_PURPLE: Color = Color::new(255, 85, 255);
    pub const YELLOW: Color = Color::new(255, 255, 85);
    pub const WHITE: Color = Color::new(255, 255, 255);

    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ParseColorError> {
        if hex.starts_with('#') {
            let [idx_r, idx_g, idx_b] = match hex.len() {
                3 => [1..2, 2..3, 3..4],
                6 => [1..3, 3..5, 5..7],
                _ => return Err(ParseColorError::InvalidHex(hex.to_owned())),
            };
            let r = u8::from_str_radix(&hex[idx_r], 16)
                .map_err(|_| ParseColorError::InvalidHex(hex.to_owned()))?;
            let g = u8::from_str_radix(&hex[idx_g], 16)
                .map_err(|_| ParseColorError::InvalidHex(hex.to_owned()))?;
            let b = u8::from_str_radix(&hex[idx_b], 16)
                .map_err(|_| ParseColorError::InvalidHex(hex.to_owned()))?;
            Ok(Color::new(r, g, b))
        } else {
            Err(ParseColorError::InvalidHex(hex.to_owned()))
        }
    }

    pub fn from_name(name: &str) -> Result<Self, ParseColorError> {
        let lowercased = name.to_lowercase();
        match lowercased.as_str() {
            "black" => Ok(Self::BLACK),
            "dark_blue" => Ok(Self::DARK_BLUE),
            "dark_green" => Ok(Self::DARK_GREEN),
            "dark_cyan" => Ok(Self::DARK_CYAN),
            "dark_red" => Ok(Self::DARK_RED),
            "dark_purple" => Ok(Self::DARK_PURPLE),
            "gold" => Ok(Self::GOLD),
            "gray" => Ok(Self::GRAY),
            "dark_gray" => Ok(Self::DARK_GRAY),
            "blue" => Ok(Self::BLUE),
            "green" => Ok(Self::GREEN),
            "aqua" => Ok(Self::AQUA),
            "red" => Ok(Self::RED),
            "light_purple" => Ok(Self::LIGHT_PURPLE),
            "yellow" => Ok(Self::YELLOW),
            "white" => Ok(Self::WHITE),
            _ => Err(ParseColorError::InvalidName(lowercased)),
        }
    }

    pub const fn name(&self) -> Option<&'static str> {
        match *self {
            Self::BLACK => Some("black"),
            Self::DARK_BLUE => Some("dark_blue"),
            Self::DARK_GREEN => Some("dark_green"),
            Self::DARK_CYAN => Some("dark_cyan"),
            Self::DARK_RED => Some("dark_red"),
            Self::DARK_PURPLE => Some("dark_purple"),
            Self::GOLD => Some("gold"),
            Self::GRAY => Some("gray"),
            Self::DARK_GRAY => Some("dark_gray"),
            Self::BLUE => Some("blue"),
            Self::GREEN => Some("green"),
            Self::AQUA => Some("aqua"),
            Self::RED => Some("red"),
            Self::LIGHT_PURPLE => Some("light_purple"),
            Self::YELLOW => Some("yellow"),
            Self::WHITE => Some("white"),
            _ => None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseColorError {
    #[error("expected hex color (#rrggbb or #rgb), got {0}")]
    InvalidHex(String),
    #[error("expected named color, got {0}")]
    InvalidName(String),
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(name_or_hex: &str) -> Result<Self, Self::Err> {
        Color::from_name(name_or_hex)
            .or_else(|_| Color::from_hex(name_or_hex))
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a color such as #000000 or named color")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error
    {
        v.parse()
            .map_err(|e| E::custom(e))
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
