#![doc = include_str!("../README.md")]

#![forbid(missing_docs)]

use std::borrow::Cow;
use std::fmt;

#[cfg(feature = "minecrevy_io_str")]
pub use self::io_str::*;

#[cfg(feature = "minecrevy_io_str")]
mod io_str;

/// The error type returned when [`Key`] validation fails.
#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    /// The error variant that represents a failed namespace validation.
    #[error("invalid namespace: {0}")]
    InvalidNamespace(Cow<'static, str>),
    /// The error variant that represents a failed path validation.
    #[error("invalid path: {0}")]
    InvalidPath(Cow<'static, str>),
}

/// A representation of a location or pointer to resources.
///
/// The key is built with two parts:
/// - Namespace
/// - Path
///
/// Normally, the namespace is lowercased and likewise, so is the path.
///
/// **Note:** the [`minecraft`][Key::minecraft] and [`minecrevy`][Key::minecrevy] constructors should only be used by
/// the implementation for creating new keys and usage by plugins **may** result in a crash condition.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Key {
    namespace: Cow<'static, str>,
    path: Cow<'static, str>,
}

impl Key {
    /// The `brigadier` namespace.
    pub const NAMESPACE_BRIGADIER: &'static str = "brigadier";

    /// The `minecraft` namespace.
    pub const NAMESPACE_MINECRAFT: &'static str = "minecraft";

    /// The `minecrevy` namespace.
    pub const NAMESPACE_MINECREVY: &'static str = "minecrevy";

    /// Creates a key with a namespace of [`brigadier`][Self::NAMESPACE_BRIGADIER], ensuring the path is valid.
    pub fn brigadier(path: impl Into<Cow<'static, str>>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Cow::Borrowed(Self::NAMESPACE_BRIGADIER),
            path: Self::validate_path(path.into())?,
        })
    }

    /// Creates a key with a namespace of [`minecraft`][Self::NAMESPACE_MINECRAFT], ensuring the path is valid.
    pub fn minecraft(path: impl Into<Cow<'static, str>>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Cow::Borrowed(Self::NAMESPACE_MINECRAFT),
            path: Self::validate_path(path.into())?,
        })
    }

    /// Creates a key with a namespace of [`minecrevy`][Self::NAMESPACE_MINECREVY], ensuring the path is valid.
    pub fn minecrevy(path: impl Into<Cow<'static, str>>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Cow::Borrowed(Self::NAMESPACE_MINECREVY),
            path: Self::validate_path(path.into())?,
        })
    }

    /// Creates a key, ensuring the namespace and path are valid.
    pub fn new(namespace: impl Into<Cow<'static, str>>, path: impl Into<Cow<'static, str>>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Self::validate_namespace(namespace.into())?,
            path: Self::validate_path(path.into())?,
        })
    }

    /// Parses a key from a string.
    ///
    /// If no namespace is found in `formatted` then the
    /// [`minecraft`][Self::NAMESPACE_MINECRAFT] namespace will be used.
    pub fn parse(formatted: impl Into<Cow<'static, str>>) -> Result<Self, KeyError> {
        Self::parse_or(formatted, Self::NAMESPACE_MINECRAFT)
    }

    /// Parses a key from a string.
    ///
    /// If no namespace is found in `formatted` then `default_namespace` will be used.
    pub fn parse_or(formatted: impl Into<Cow<'static, str>>, default_namespace: impl Into<Cow<'static, str>>) -> Result<Self, KeyError> {
        let formatted = formatted.into();
        if let Some((namespace, path)) = formatted.split_once(':') {
            Key::new(namespace.to_owned(), path.to_owned())
        } else {
            Key::new(formatted, default_namespace.into())
        }
    }

    /// Creates a key without validating the namespace or path.
    ///
    /// # Safety
    /// `namespace` and `path` must be valid (`[a-zA-Z0-9._-]*` and `[a-zA-Z0-9._-/]*`, respectively).
    pub const unsafe fn new_const(namespace: &'static str, path: &'static str) -> Self {
        Self {
            namespace: Cow::Borrowed(namespace),
            path: Cow::Borrowed(path),
        }
    }

    /// Gets the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Gets the path.
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Key {
    fn validate_namespace(namespace: Cow<'static, str>) -> Result<Cow<'static, str>, KeyError> {
        fn is_namespace_char(c: char) -> bool {
            matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-')
        }

        if namespace.chars().all(is_namespace_char) {
            Ok(namespace)
        } else {
            Err(KeyError::InvalidNamespace(namespace))
        }
    }

    fn validate_path(path: Cow<'static, str>) -> Result<Cow<'static, str>, KeyError> {
        fn is_path_char(c: char) -> bool {
            matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' | '/')
        }

        if path.chars().all(is_path_char) {
            Ok(path)
        } else {
            Err(KeyError::InvalidPath(path))
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
