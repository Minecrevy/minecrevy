use std::{
    borrow::Borrow,
    cmp::{Ordering, PartialEq},
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
};

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
pub enum Bow<'a, B: ?Sized + 'a>
where
    B: ToBoxed,
{
    /// Boxed data.
    Boxed(B::Boxed),
    /// Borrowed data.
    Borrowed(&'a B),
}

impl<B: ?Sized + ToBoxed> Clone for Bow<'_, B> {
    fn clone(&self) -> Self {
        match *self {
            Bow::Borrowed(borrowed) => Bow::Borrowed(borrowed),
            Bow::Boxed(ref o) => {
                let b: &B = o.borrow();
                Bow::Boxed(b.to_boxed())
            }
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (&mut Bow::Boxed(ref mut dest), &Bow::Boxed(ref o)) => o.borrow().box_into(dest),
            (t, s) => *t = s.clone(),
        }
    }
}

impl<B: ?Sized + ToBoxed> Bow<'_, B> {
    /// Returns true if the data is borrowed, i.e. if `to_mut` would require additional work.
    pub const fn is_borrowed(&self) -> bool {
        match *self {
            Bow::Borrowed(_) => true,
            Bow::Boxed(_) => false,
        }
    }

    /// Returns true if the data is boxed, i.e. if `to_mut` would be a no-op.
    pub const fn is_boxed(&self) -> bool {
        !self.is_borrowed()
    }

    /// Acquires a mutable reference to the boxed form of the data.
    ///
    /// Clones and boxes the data if it is not already owned.
    pub fn to_mut(&mut self) -> &mut B::Boxed {
        match *self {
            Bow::Borrowed(borrowed) => {
                *self = Bow::Boxed(borrowed.to_boxed());
                match *self {
                    Bow::Borrowed(_) => unreachable!(),
                    Bow::Boxed(ref mut boxed) => boxed,
                }
            }
            Bow::Boxed(ref mut boxed) => boxed,
        }
    }

    /// Extracts the boxed data.
    ///
    /// Clones and boxes the data if it is not already owned.
    pub fn into_boxed(self) -> B::Boxed {
        match self {
            Bow::Borrowed(borrowed) => borrowed.to_boxed(),
            Bow::Boxed(boxed) => boxed,
        }
    }
}

impl<B: ?Sized + ToBoxed> Deref for Bow<'_, B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        match *self {
            Bow::Borrowed(b) => b,
            Bow::Boxed(ref boxed) => boxed.borrow(),
        }
    }
}

impl<B: ?Sized> Eq for Bow<'_, B> where B: Eq + ToBoxed {}

impl<'a, 'b, B: ?Sized, C: ?Sized> PartialEq<Bow<'b, C>> for Bow<'a, B>
where
    B: PartialEq<C> + ToBoxed,
    C: ToBoxed,
{
    #[inline]
    fn eq(&self, other: &Bow<'b, C>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<B: ?Sized> Ord for Bow<'_, B>
where
    B: Ord + ToBoxed,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<'a, B: ?Sized> PartialOrd for Bow<'a, B>
where
    B: PartialOrd + ToBoxed,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<B: ?Sized> fmt::Debug for Bow<'_, B>
where
    B: fmt::Debug + ToBoxed,
    B::Boxed: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Bow::Borrowed(ref b) => fmt::Debug::fmt(b, f),
            Bow::Boxed(ref o) => fmt::Debug::fmt(o, f),
        }
    }
}

impl<B: ?Sized> fmt::Display for Bow<'_, B>
where
    B: fmt::Display + ToBoxed,
    B::Boxed: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Bow::Borrowed(ref b) => fmt::Display::fmt(b, f),
            Bow::Boxed(ref o) => fmt::Display::fmt(o, f),
        }
    }
}

impl<B: ?Sized> Default for Bow<'_, B>
where
    B: ToBoxed,
    B::Boxed: Default,
{
    fn default() -> Self {
        Bow::Boxed(B::Boxed::default())
    }
}

impl<B: ?Sized> Hash for Bow<'_, B>
where
    B: Hash + ToBoxed,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T: ?Sized + ToBoxed> AsRef<T> for Bow<'_, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T: ?Sized> Serialize for Bow<'a, T>
where
    T: Serialize + ToBoxed,
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
    T: ToBoxed,
    T::Boxed: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::Boxed::deserialize(deserializer).map(Bow::Boxed)
    }
}

pub trait ToBoxed {
    type Boxed: Borrow<Self>;

    fn to_boxed(&self) -> Self::Boxed;

    fn box_into(&self, target: &mut Self::Boxed) {
        *target = self.to_boxed();
    }
}

impl<T> ToBoxed for T
where
    T: Clone,
{
    type Boxed = Box<T>;

    #[inline]
    fn to_boxed(&self) -> Self::Boxed {
        Box::new(self.clone())
    }

    fn box_into(&self, target: &mut Self::Boxed) {
        (**target).clone_from(self);
    }
}

impl ToBoxed for str {
    type Boxed = String;

    #[inline]
    fn to_boxed(&self) -> Self::Boxed {
        self.to_owned()
    }

    fn box_into(&self, target: &mut Self::Boxed) {
        self.clone_into(target)
    }
}

impl<T> ToBoxed for [T]
where
    T: Clone,
{
    type Boxed = Vec<T>;

    fn to_boxed(&self) -> Self::Boxed {
        self.to_vec()
    }

    fn box_into(&self, target: &mut Self::Boxed) {
        self.clone_into(target);
    }
}
