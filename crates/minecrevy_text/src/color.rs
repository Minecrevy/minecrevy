use std::{fmt, str::FromStr};

use minecrevy_core::color::{Hsv, HsvLike, RgbLike};
use ordered_float::OrderedFloat;
use thiserror::Error;

/// RGB-based color of text.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TextColor {
    /// The red color channel.
    pub red: u8,
    /// The green color channel.
    pub green: u8,
    /// The blue color channel.
    pub blue: u8,
}

impl TextColor {
    /// The 16 named colors.
    pub const NAMED: [TextColor; 16] = [
        Self::BLACK,
        Self::DARK_BLUE,
        Self::DARK_GREEN,
        Self::DARK_AQUA,
        Self::DARK_RED,
        Self::DARK_PURPLE,
        Self::GOLD,
        Self::GRAY,
        Self::DARK_GRAY,
        Self::BLUE,
        Self::GREEN,
        Self::AQUA,
        Self::RED,
        Self::LIGHT_PURPLE,
        Self::YELLOW,
        Self::WHITE,
    ];

    /// The named color 'black'.
    pub const BLACK: TextColor = TextColor::new(0, 0, 0);
    /// The named color 'dark_blue'.
    pub const DARK_BLUE: TextColor = TextColor::new(0, 0, 170);
    /// The named color 'dark_green'.
    pub const DARK_GREEN: TextColor = TextColor::new(0, 170, 0);
    /// The named color 'dark_aqua'.
    pub const DARK_AQUA: TextColor = TextColor::new(0, 170, 170);
    /// The named color 'dark_red'.
    pub const DARK_RED: TextColor = TextColor::new(170, 0, 0);
    /// The named color 'dark_purple'.
    pub const DARK_PURPLE: TextColor = TextColor::new(170, 0, 170);
    /// The named color 'gold'.
    pub const GOLD: TextColor = TextColor::new(255, 170, 0);
    /// The named color 'gray'.
    pub const GRAY: TextColor = TextColor::new(170, 170, 170);
    /// The named color 'dark_gray'.
    pub const DARK_GRAY: TextColor = TextColor::new(85, 85, 85);
    /// The named color 'blue'.
    pub const BLUE: TextColor = TextColor::new(85, 85, 255);
    /// The named color 'green'.
    pub const GREEN: TextColor = TextColor::new(85, 255, 85);
    /// The named color 'aqua'.
    pub const AQUA: TextColor = TextColor::new(85, 255, 255);
    /// The named color 'red'.
    pub const RED: TextColor = TextColor::new(255, 85, 85);
    /// The named color 'light_purple'.
    #[doc(alias = "pink")]
    pub const LIGHT_PURPLE: TextColor = TextColor::new(255, 85, 255);
    /// The named color 'yellow'.
    pub const YELLOW: TextColor = TextColor::new(255, 255, 85);
    /// The named color 'white'.
    pub const WHITE: TextColor = TextColor::new(255, 255, 255);

    /// Constructs a new color with the provided red, green, and blue channels.
    /// Const-enabled.
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Returns the hexadecimal representation of this color.
    pub fn hex_string(&self) -> String {
        let TextColor { red, green, blue } = *self;
        format!("#{red:02x}{green:02x}{blue:02x}")
    }

    /// Returns the color that matches the specified name, if it exists.
    pub fn from_name(name: &str) -> Option<TextColor> {
        match name {
            "black" => Some(Self::BLACK),
            "dark_blue" => Some(Self::DARK_BLUE),
            "dark_green" => Some(Self::DARK_GREEN),
            "dark_aqua" => Some(Self::DARK_AQUA),
            "dark_red" => Some(Self::DARK_RED),
            "dark_purple" => Some(Self::DARK_PURPLE),
            "gold" => Some(Self::GOLD),
            "gray" => Some(Self::GRAY),
            "dark_gray" => Some(Self::DARK_GRAY),
            "blue" => Some(Self::BLUE),
            "green" => Some(Self::GREEN),
            "aqua" => Some(Self::AQUA),
            "red" => Some(Self::RED),
            "light_purple" => Some(Self::LIGHT_PURPLE),
            "yellow" => Some(Self::YELLOW),
            "white" => Some(Self::WHITE),
            _ => None,
        }
    }

    /// Returns the name that matches the color, if it exists.
    pub const fn name(&self) -> Option<&'static str> {
        match *self {
            Self::BLACK => Some("black"),
            Self::DARK_BLUE => Some("dark_blue"),
            Self::DARK_GREEN => Some("dark_green"),
            Self::DARK_AQUA => Some("dark_aqua"),
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

    /// Returns the nearest named color to the current color.
    pub fn nearest_named(&self) -> Self {
        fn distance(a: impl HsvLike, b: impl HsvLike) -> f32 {
            let dh = 3.0 * f32::min((a.h() - b.h()).abs(), 1.0 - (a.h() - b.h()).abs());
            let ds = a.s() - b.s();
            let dv = a.v() - b.v();

            dh * dh + ds * ds + dv * dv
        }

        if self.name().is_some() {
            return *self;
        }

        Self::NAMED
            .into_iter()
            .min_by_key(|color| {
                let distance = distance(self.into_hsv::<Hsv>(), color.into_hsv::<Hsv>());
                OrderedFloat(distance)
            })
            .unwrap_or(Self::BLACK)
    }
}

impl RgbLike for TextColor {
    fn red(&self) -> u8 {
        self.red
    }

    fn green(&self) -> u8 {
        self.green
    }

    fn blue(&self) -> u8 {
        self.blue
    }

    fn new(red: u8, green: u8, blue: u8) -> Self
    where
        Self: Sized,
    {
        TextColor::new(red, green, blue)
    }
}

impl Default for TextColor {
    fn default() -> Self {
        Self::WHITE
    }
}

/// An error type for conversion from [`String`].
#[derive(Error, Debug)]
pub enum TextColorFromError {
    /// An unknown named, or invalid hexadecimal color was given.
    #[error("color must be hexadecimal or a specific name: {0}")]
    Unknown(String),
}

impl FromStr for TextColor {
    type Err = TextColorFromError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(named) = Self::from_name(value) {
            Ok(named)
        } else if value.starts_with('#') {
            let rgb = u32::from_str_radix(&value[1..], 16)
                .map_err(|_| TextColorFromError::Unknown(value.to_string()))?;
            Ok(Self::new_rgb(rgb))
        } else {
            Err(TextColorFromError::Unknown(value.to_string()))
        }
    }
}

impl fmt::Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = self.name() {
            write!(f, "{}", name)
        } else {
            let TextColor { red, green, blue } = self;
            write!(f, "#{red:02x}{green:02x}{blue:02x}")
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use serde::{de::Visitor, Deserialize, Serialize};

    use crate::TextColor;

    impl Serialize for TextColor {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for TextColor {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(TextColorVisitor)
        }
    }

    struct TextColorVisitor;

    impl<'de> Visitor<'de> for TextColorVisitor {
        type Value = TextColor;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "an RGB color in hexadecimal format, or a named color")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse().map_err(|e| serde::de::Error::custom(e))
        }
    }
}
