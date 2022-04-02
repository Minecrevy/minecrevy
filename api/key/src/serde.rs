use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

use crate::Key;

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_string(KeyVisitor)
    }
}

struct KeyVisitor;

impl<'de> Visitor<'de> for KeyVisitor {
    type Value = Key;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("key of format <namespace>:<value>")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Key::parse(v)
            .map_err(|e| E::custom(e))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
        Key::parse(v)
            .map_err(|e| E::custom(e))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Key::parse(v)
            .map_err(|e| E::custom(e))
    }


}
