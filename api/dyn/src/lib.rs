#![doc = include_str!("../README.md")]

#![forbid(missing_docs)]

/// Allows `Clone` in an object-safe context.
///
/// # Usage
/// ```rust
/// pub trait Foo: FooClone {}
///
/// minecrevy_dyn::dyn_clone!(Foo: FooClone); // The trait definition should match this.
/// ```
#[macro_export]
macro_rules! dyn_clone {
    ($ty:ident: $ty_clone:ident) => {
        /// A trait for object-safe cloning.
        pub trait $ty_clone {
            /// Returns a copy of the value as a trait object.
            fn clone_boxed(&self) -> Box<dyn $ty>;
        }

        impl<T: $ty + Clone + 'static> $ty_clone for T {
            fn clone_boxed(&self) -> Box<dyn $ty> {
                Box::new(self.clone())
            }
        }

        impl Clone for Box<dyn $ty> {
            fn clone(&self) -> Self {
                self.clone_boxed()
            }
        }
    };
}

/// Allows `PartialEq` in an object-safe context.
///
/// # Usage
/// ```rust
/// pub trait Foo: FooPartialEq {}
///
/// minecrevy_dyn::dyn_partial_eq!(Foo: FooPartialEq); // The trait definition should match this.
/// ```
#[macro_export]
macro_rules! dyn_partial_eq {
    ($ty:ident: $ty_partial_eq:ident) => {
        /// A trait for object-safe equality testing.
        pub trait $ty_partial_eq {
            /// This method tests for self and other values to be equal in a dynamic manner.
            fn eq_dyn(&self, other: &dyn $ty) -> bool;
        }

        impl<T: $ty + PartialEq + 'static> $ty_partial_eq for T {
            fn eq_dyn(&self, other: &dyn $ty) -> bool {
                if let Some(other) = other.downcast_ref() {
                    PartialEq::eq(self, other)
                } else {
                    false
                }
            }
        }

        impl PartialEq for dyn $ty {
            fn eq(&self, other: &Self) -> bool {
                self.eq_dyn(&*other)
            }
        }
    };
}
