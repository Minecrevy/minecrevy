/// Implements [`PartialEq`] and [`Eq`] for a type based on its field of type [`Key`][`crate::Key`].
///
/// # Example
/// ```rust
/// use minecrevy_key::{Key, equality_by_key};
///
/// pub struct Foo {
///     pub id: Key,
/// }
///
/// equality_by_key!(Foo: id);
/// ```
#[macro_export]
macro_rules! equality_by_key {
    ($ty:ty : $field:ident) => {
        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self.$field.eq(&other.$field)
            }
        }

        impl Eq for $ty {}
    };
}

/// Implements [`PartialOrd`] and [`Ord`] for a type based on its field of type [`Key`][`crate::Key`].
///
/// # Example
/// ```rust
/// use minecrevy_key::{Key, ordered_by_key};
///
/// pub struct Foo {
///     pub id: Key,
/// }
///
/// ordered_by_key!(Foo: id);
/// ```
#[macro_export]
macro_rules! ordered_by_key {
    ($ty:ty : $field:ident) => {
        impl PartialOrd for $ty {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.key.partial_cmp(&other.$field)
            }
        }

        impl Ord for $ty {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.$field.cmp(&other.$field)
            }
        }
    };
}
