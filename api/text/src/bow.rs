use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A box-on-write smart pointer.
///
/// The type `Bow` is a smart pointer providing box-on-write functionality: it
/// can enclose and provide immutable access to borrowed data, and box the
/// data lazily when mutation or ownership is required.
///
/// `Bow` implements `Deref`, which means that you can call
/// non-mutating methods directly on the data it encloses. If mutation
/// is desired, `to_mut` will obtain a mutable reference to an owned
/// value, boxing if necessary.
pub enum Bow<'a, T: ?Sized + 'a> {
    Borrowed(&'a T),
    Boxed(Box<T>),
}

impl<T: ?Sized> Bow<'_, T> {
    pub fn is_borrowed(&self) -> bool {
        match self {
            Bow::Borrowed(_) => true,
            Bow::Boxed(_) => false,
        }
    }

    pub fn is_boxed(&self) -> bool {
        !self.is_borrowed()
    }
}

impl<T: ?Sized> Bow<'_, T>
where
    T: Clone
{
    pub fn to_mut(&mut self) -> &mut T {
        match *self {
            Bow::Borrowed(b) => {
                *self = Bow::Boxed(Box::new(b.clone()));
                match *self {
                    Bow::Borrowed(_) => unreachable!(),
                    Bow::Boxed(ref mut boxed) => boxed,
                }
            }
            Bow::Boxed(ref mut boxed) => boxed,
        }
    }

    pub fn into_boxed(self) -> Box<T> {
        match self {
            Bow::Borrowed(b) => Box::new(b.clone()),
            Bow::Boxed(boxed) => boxed,
        }
    }
}

impl<T: ?Sized> Deref for Bow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Bow::Borrowed(b) => b,
            Bow::Boxed(ref boxed) => boxed,
        }
    }
}

impl<T: ?Sized> Clone for Bow<'_, T>
where
    T: Clone
{
    fn clone(&self) -> Self {
        match *self {
            Bow::Borrowed(b) => Bow::Borrowed(b),
            Bow::Boxed(ref o) => {
                let b = o.clone();
                Bow::Boxed(b)
            }
        }
    }
}

impl<T: ?Sized> Eq for Bow<'_, T> where T: PartialEq + Eq {}

impl<'a, 'b, T: ?Sized, C: ?Sized> PartialEq<Bow<'b, C>> for Bow<'a, T>
where
    T: PartialEq<C>
{
    #[inline]
    fn eq(&self, other: &Bow<'b, C>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<T: ?Sized> Ord for Bow<'_, T> where T: PartialOrd + Ord {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<'a, T: ?Sized> PartialOrd for Bow<'a, T>
where
    T: PartialOrd
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<T: ?Sized> fmt::Debug for Bow<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Bow::Borrowed(ref b) => fmt::Debug::fmt(b, f),
            Bow::Boxed(ref o) => fmt::Debug::fmt(o, f),
        }
    }
}

impl<T: ?Sized> fmt::Display for Bow<'_, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Bow::Borrowed(ref b) => fmt::Display::fmt(b, f),
            Bow::Boxed(ref o) => fmt::Display::fmt(o, f),
        }
    }
}

impl<T: ?Sized> Default for Bow<'_, T>
where
    T: Default
{
    fn default() -> Self {
        Bow::Boxed(Box::new(T::default()))
    }
}

impl<T: ?Sized> Hash for Bow<'_, T>
where
    T: Hash
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T: ?Sized> AsRef<T> for Bow<'_, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T: ?Sized> Serialize for Bow<'a, T>
where
    T: Serialize
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<'de, 'a, T: ?Sized> Deserialize<'de> for Bow<'a, T>
where
    T: Deserialize<'de>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        T::deserialize(deserializer).map(Box::new).map(Bow::Boxed)
    }
}
