use flexstr::SharedStr;
use serde::{Deserialize, Serialize};

use crate::Style;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextValue {
    String {
        text: SharedStr,
    },
    Translation {
        translate: SharedStr,
        with: Vec<SharedStr>,
    },
    Keybind {
        keybind: SharedStr,
    },
}

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
    pub fn string(text: impl Into<SharedStr>, style: Style) -> Self {
        Self {
            value: TextValue::String { text: text.into() },
            style,
            extra: vec![],
        }
    }

    pub const fn str(text: &'static str, style: Style) -> Self {
        Self {
            value: TextValue::String {
                text: SharedStr::from_static(text),
            },
            style,
            extra: vec![],
        }
    }

    #[inline]
    pub const fn empty() -> Self {
        Self::str("", Style::empty())
    }

    #[inline]
    pub const fn space() -> Self {
        Self::str(" ", Style::empty())
    }

    #[inline]
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

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Text::string(s, Style::empty())
    }
}
