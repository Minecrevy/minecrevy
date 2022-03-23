#![doc = include_str ! ("../README.md")]

#![warn(missing_docs)]

use std::fmt;

use flexstr::{shared_str, SharedStr};

pub use minecrevy_key_macros::key;

#[cfg(feature = "minecrevy_io_str")]
pub use self::io_str::*;
pub use self::macros::*;
#[cfg(feature = "serde")]
pub use self::serde::*;

#[cfg(feature = "minecrevy_io_str")]
mod io_str;
mod macros;
#[cfg(feature = "serde")]
mod serde;

/// The error type returned when [`Key`] validation fails.
#[derive(thiserror::Error, Debug)]
pub enum KeyError {
    /// The error variant that represents a failed namespace validation.
    #[error("non [a-z0-9_.-] character in namespace: {0}")]
    InvalidNamespace(SharedStr),
    /// The error variant that represents a failed path validation.
    #[error("non [a-z0-9_.-/] character in path: {0}")]
    InvalidPath(SharedStr),
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
/// the implementation for creating new keys and usage by servers **may** result in a crash condition.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Key {
    namespace: SharedStr,
    path: SharedStr,
}

impl Key {
    /// The `brigadier` namespace.
    pub const NAMESPACE_BRIGADIER: SharedStr = shared_str!("brigadier");

    /// The `minecraft` namespace.
    pub const NAMESPACE_MINECRAFT: SharedStr = shared_str!("minecraft");

    /// The `minecrevy` namespace.
    pub const NAMESPACE_MINECREVY: SharedStr = shared_str!("minecrevy");

    /// Creates a key with a namespace of [`brigadier`][Self::NAMESPACE_BRIGADIER], ensuring the path is valid.
    pub fn brigadier(path: impl Into<SharedStr>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Self::NAMESPACE_BRIGADIER,
            path: Self::validate_path(path.into())?,
        })
    }

    /// Creates a key with a namespace of [`minecraft`][Self::NAMESPACE_MINECRAFT], ensuring the path is valid.
    pub fn minecraft(path: impl Into<SharedStr>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Self::NAMESPACE_MINECRAFT,
            path: Self::validate_path(path.into())?,
        })
    }

    /// Creates a key with a namespace of [`minecrevy`][Self::NAMESPACE_MINECREVY], ensuring the path is valid.
    pub fn minecrevy(path: impl Into<SharedStr>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Self::NAMESPACE_MINECREVY,
            path: Self::validate_path(path.into())?,
        })
    }

    /// Creates a key, ensuring the namespace and path are valid.
    pub fn new(namespace: impl Into<SharedStr>, path: impl Into<SharedStr>) -> Result<Self, KeyError> {
        Ok(Self {
            namespace: Self::validate_namespace(namespace.into())?,
            path: Self::validate_path(path.into())?,
        })
    }

    /// Creates a key without validating the namespace or path.
    ///
    /// # Safety
    /// `namespace` and `path` must be valid (`[a-z0-9._-]*` and `[a-z0-9._-/]*`, respectively).
    pub const unsafe fn new_unchecked(namespace: SharedStr, path: SharedStr) -> Self {
        Self {
            namespace,
            path,
        }
    }

    /// Creates a key without validating the namespace or path. Used by the [`key`] macro.
    ///
    /// # Safety
    /// `namespace` and `path` must be valid (`[a-z0-9._-]*` and `[a-z0-9._-/]*`, respectively).
    pub const unsafe fn static_unchecked(namespace: &'static str, path: &'static str) -> Self {
        Self::new_unchecked(SharedStr::from_static(namespace), SharedStr::from_static(path))
    }

    /// Parses a key from a string.
    ///
    /// If no namespace is found in `formatted` then the
    /// [`minecraft`][Self::NAMESPACE_MINECRAFT] namespace will be used.
    pub fn parse(formatted: impl Into<SharedStr>) -> Result<Self, KeyError> {
        Self::parse_or(formatted, Self::NAMESPACE_MINECRAFT)
    }

    /// Parses a key from a string.
    ///
    /// If no namespace is found in `formatted` then `default_namespace` will be used.
    pub fn parse_or(formatted: impl Into<SharedStr>, default_namespace: impl Into<SharedStr>) -> Result<Self, KeyError> {
        let formatted = formatted.into();
        if let Some((namespace, path)) = formatted.split_once(':') {
            Key::new(SharedStr::from(namespace), SharedStr::from(path))
        } else {
            Key::new(default_namespace.into(), formatted)
        }
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the key as a (`namespace`, `path`) pair.
    pub fn as_parts(&self) -> (&str, &str) {
        (&self.namespace, &self.path)
    }

    /// Creates a [`KeyRef`], useful for const pattern matching.
    pub fn as_ref(&self) -> KeyRef {
        KeyRef {
            namespace: self.namespace(),
            path: self.path(),
        }
    }
}

impl Key {
    fn validate_namespace(namespace: SharedStr) -> Result<SharedStr, KeyError> {
        fn is_namespace_char(c: char) -> bool {
            matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-')
        }

        if namespace.is_empty() {
            Ok(Self::NAMESPACE_MINECRAFT)
        } else if namespace.chars().all(is_namespace_char) {
            Ok(namespace)
        } else {
            Err(KeyError::InvalidNamespace(namespace))
        }
    }

    fn validate_path(path: SharedStr) -> Result<SharedStr, KeyError> {
        fn is_path_char(c: char) -> bool {
            matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/')
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

/// A `'static` [`KeyRef`].
pub type StaticKey = KeyRef<'static>;

/// A reference to a key's parts. Used for const pattern matching.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct KeyRef<'a> {
    namespace: &'a str,
    path: &'a str,
}

impl<'a> KeyRef<'a> {
    /// Gets the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Gets the path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Creates a reference key without validating the namespace or path.
    ///
    /// # Safety
    /// `namespace` and `path` must be valid (`[a-z0-9._-]*` and `[a-z0-9._-/]*`, respectively).
    pub const unsafe fn static_unchecked(namespace: &'a str, path: &'a str) -> Self {
        Self {
            namespace,
            path,
        }
    }
}

impl<'a> fmt::Display for KeyRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}
