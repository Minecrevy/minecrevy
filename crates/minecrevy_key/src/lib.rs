use std::{
    fmt::{Debug, Display},
    io,
    ops::Deref,
    str::FromStr,
};

use flexstr::{LocalStr, SharedStr};
use minecrevy_io::{args::StringArgs, McRead, McWrite};
use serde::{de::Visitor, Deserialize, Serialize, Serializer};
use thiserror::Error;

/// A [`Key`] backed by a [`SharedStr`].
pub type SharedKey = Key<SharedStr>;

/// A [`Key`] backed by a [`LocalStr`].
pub type LocalKey = Key<LocalStr>;

/// An object used to fetch and/or store unique Minecraft objects.
///
/// A key consists of:
/// - `namespace`: minecraft, your plugin name, organization name, etc
/// - `path`: how to find the resource, like `entity.firework_rocket.blast`
///
/// Valid characters for:
/// - `namespace`: `[a-z0-9_.-]`
/// - `path`: `[a-z0-9_.-/]`
#[cfg_attr(
    feature = "bevy",
    derive(bevy::prelude::Component, bevy::prelude::Reflect)
)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key<S> {
    namespace: S,
    path: S,
}

impl<S> Key<S> {
    /// The default namespace, Minecraft's.
    pub const MINECRAFT_NAMESPACE: &'static str = "minecraft";

    /// The default namespace and path separator.
    pub const DEFAULT_SEPARATOR: char = ':';
}

impl<S: Deref<Target = str>> Key<S> {
    /// Constructs a key given the namespace and path.
    pub fn new(namespace: impl Into<S>, path: impl Into<S>) -> Result<Self, KeyError<S>> {
        Ok(Self {
            namespace: Self::validate_namespace(namespace.into())?,
            path: Self::validate_path(path.into())?,
        })
    }

    /// Parses a key from the given string and separator.
    ///
    /// The namespace is optional; it will default to [`minecraft`](Self::MINECRAFT_NAMESPACE) if not specified.
    pub fn parse(key: impl Into<S>, separator: char) -> Result<Self, KeyError<S>>
    where
        for<'a> S: From<&'a str>,
    {
        let key: S = key.into();
        if let Some((namespace, path)) = key.split_once(separator) {
            Self::new(namespace, path)
        } else {
            Self::new(Self::MINECRAFT_NAMESPACE, key)
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
}

impl<S: Display> Key<S> {
    /// Returns the key as a string, using the given separator.
    pub fn as_string(&self, separator: char) -> String {
        format!("{}{}{}", self.namespace, separator, self.path)
    }
}

impl<S: Deref<Target = str>> Key<S> {
    fn validate_namespace(namespace: S) -> Result<S, KeyError<S>> {
        // TODO: we can switch to ascii-only iteration eventually: https://github.com/rust-lang/rust/issues/110998
        // currently this iterates UTF-8 which might not auto vectorize as well
        for c in namespace.chars() {
            if !matches!(c, '_' | '-' | 'a'..='z' | '0'..='9' | '.') {
                return Err(KeyError::InvalidNamespace(namespace));
            }
        }

        Ok(namespace)
    }

    fn validate_path(path: S) -> Result<S, KeyError<S>> {
        // TODO: same as validate_namespace
        for c in path.chars() {
            if !matches!(c, '_' | '-' | 'a'..='z' | '0'..='9' | '.' | '/') {
                return Err(KeyError::InvalidPath(path));
            }
        }

        Ok(path)
    }
}

impl<S: Deref<Target = str>> FromStr for Key<S>
where
    for<'a> S: From<&'a str>,
{
    type Err = KeyError<S>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, ':')
    }
}

impl<S: Display> Display for Key<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_string(Self::DEFAULT_SEPARATOR))
    }
}

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum KeyError<S> {
    #[error("Non [a-z0-9_.-] character in namespace: {0}")]
    InvalidNamespace(S),
    #[error("Non [a-z0-9_.-/] character in path: {0}")]
    InvalidPath(S),
}

impl<Str: Display> Serialize for Key<Str> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct KeyVisitor<S>(std::marker::PhantomData<S>);

impl<S> Visitor<'_> for KeyVisitor<S>
where
    S: Deref<Target = str> + Display + for<'a> From<&'a str>,
{
    type Value = Key<S>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string of the form `namespace:path`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Key::parse(v, Key::<S>::DEFAULT_SEPARATOR).map_err(|e| E::custom(e))
    }
}

impl<'de, S> Deserialize<'de> for Key<S>
where
    S: Deref<Target = str> + Display + for<'a> From<&'a str>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyVisitor(std::marker::PhantomData))
    }
}

/// Arguments for reading and writing [`Key`]s.
#[derive(Clone, Debug)]
pub struct KeyArgs {
    /// The separator between the namespace and path.
    pub separator: char,
    /// Whether to skip the namespace when serializing.
    pub skip_namespace: bool,
}

impl Default for KeyArgs {
    fn default() -> Self {
        Self {
            separator: ':',
            skip_namespace: false,
        }
    }
}

impl<
        S: Deref<Target = str>
            + From<String>
            + for<'a> From<&'a str>
            + Debug
            + Display
            + Send
            + Sync
            + 'static,
    > McRead for Key<S>
{
    type Args = KeyArgs;

    fn read(reader: impl io::Read, args: Self::Args) -> io::Result<Self> {
        let key = String::read(
            reader,
            StringArgs {
                max_len: Some(32767),
            },
        )?;
        Ok(Self::parse(key, args.separator)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
    }
}

impl<S: Deref<Target = str> + Display> McWrite for Key<S> {
    type Args = KeyArgs;

    fn write(&self, writer: impl io::Write, args: Self::Args) -> io::Result<()> {
        if args.skip_namespace {
            self.path().write(
                writer,
                StringArgs {
                    max_len: Some(32767),
                },
            )?;
        } else {
            self.as_string(args.separator).write(
                writer,
                StringArgs {
                    max_len: Some(32767),
                },
            )?;
        };

        Ok(())
    }
}
