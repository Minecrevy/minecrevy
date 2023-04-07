use std::{
    fmt::{self, Display},
    str::FromStr,
};

use crate::str::{CompactString, ToCompactString};
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Key {
    namespace: CompactString,
    path: CompactString,
}

#[derive(Error, Clone, PartialEq, Eq, Debug, Hash)]
pub enum KeyError {
    #[error("namespace must contain only [a-z0-9_.-] but found: {0}")]
    InvalidNamespace(CompactString),
    #[error("path must contain only [a-z0-9_.-/] but found: {0}")]
    InvalidPath(CompactString),
}

impl Key {
    /// The default `namespace` and `path` separator.
    pub const DEFAULT_SEPARATOR: char = ':';
    /// The namespace for vanilla Minecraft resources.
    pub const MINECRAFT_NAMESPACE: &'static str = "minecraft";

    pub fn parse(key: &str, separator: char) -> Result<Self, KeyError> {
        if let Some((namespace, path)) = key.split_once(separator) {
            let namespace = if namespace.is_empty() {
                Self::MINECRAFT_NAMESPACE
            } else {
                namespace
            };
            Self::new(namespace.to_compact_string(), path.to_compact_string())
        } else {
            Self::new(Self::MINECRAFT_NAMESPACE, key.to_compact_string())
        }
    }

    pub fn new(
        namespace: impl Into<CompactString>,
        path: impl Into<CompactString>,
    ) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Self::validate_namespace(namespace.into())?,
            path: Self::validate_path(path.into())?,
        })
    }

    pub const unsafe fn new_unchecked(namespace: &str, path: &str) -> Self {
        Self {
            namespace: CompactString::new_inline(namespace),
            path: CompactString::new_inline(path),
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    fn validate_namespace(namespace: CompactString) -> Result<CompactString, KeyError> {
        for c in namespace.chars() {
            if matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-') {
                continue;
            } else {
                return Err(KeyError::InvalidNamespace(namespace));
            }
        }
        Ok(namespace)
    }

    fn validate_path(path: CompactString) -> Result<CompactString, KeyError> {
        for c in path.chars() {
            if matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/') {
                continue;
            } else {
                return Err(KeyError::InvalidPath(path));
            }
        }
        Ok(path)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for Key {
    type Err = KeyError;

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        Self::parse(key, Key::DEFAULT_SEPARATOR)
    }
}

#[cfg(feature = "serde")]
mod serde {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use crate::key::Key;

    impl Serialize for Key {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for Key {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(KeyVisitor)
        }
    }

    struct KeyVisitor;

    impl<'de> Visitor<'de> for KeyVisitor {
        type Value = Key;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a key with string representation <namespace>:<path>")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.parse().map_err(|e| Error::custom(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::key::{Key, KeyError};

    #[test]
    fn value_only() {
        let key: Key = "empty".parse().unwrap();
        assert_eq!(Key::MINECRAFT_NAMESPACE, key.namespace());
        assert_eq!("empty", key.path());
    }

    #[test]
    fn namespace_and_value() {
        let key: Key = Key::new(Key::MINECRAFT_NAMESPACE, "empty").unwrap();
        assert_eq!(Key::MINECRAFT_NAMESPACE, key.namespace());
        assert_eq!("empty", key.path());
    }

    #[test]
    fn namespace_and_value_parsed() {
        let key: Key = format!("{}:{}", Key::MINECRAFT_NAMESPACE, "empty")
            .parse()
            .unwrap();
        assert_eq!(Key::MINECRAFT_NAMESPACE, key.namespace());
        assert_eq!("empty", key.path());
    }

    #[test]
    fn invalid() {
        assert!(matches!(Key::from_str("!"), Err(KeyError::InvalidPath(_))));
        assert!(matches!(
            Key::from_str("Thing:abc"),
            Err(KeyError::InvalidNamespace(_))
        ));
        assert!(matches!(
            Key::from_str("abc:Thing"),
            Err(KeyError::InvalidPath(_))
        ));
        assert!(matches!(
            Key::from_str("a/b:empty"),
            Err(KeyError::InvalidNamespace(_))
        ));
    }

    #[test]
    fn string_representation() {
        assert_eq!(
            "minecraft:empty",
            Key::from_str("empty").unwrap().to_string()
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            Key::new("minecraft", "air").unwrap(),
            Key::from_str("air").unwrap()
        );
        assert_eq!(
            Key::new("minecraft", "air").unwrap(),
            Key::from_str("minecraft:air").unwrap()
        );
    }

    #[test]
    fn comparison() {
        assert!(Key::from_str("air").unwrap() < Key::from_str("stone").unwrap());
        assert_eq!(
            Key::from_str("empty").unwrap(),
            Key::from_str("empty").unwrap()
        );
        assert!(Key::from_str("stone").unwrap() > Key::from_str("air").unwrap());
    }
}
