# Minecrevy Dyn

Utilities for working with traits and trait objects.

## `Clone` but object-safe

Using the `dyn_clone` macro allows you to make your trait cloneable with a single line of code:

```rust
pub trait Foo: FooClone {}

minecrevy_dyn::dyn_clone!(Foo: FooClone);

#[derive(Clone)] // Without this, we can't implement Foo!
pub struct Bar {}

impl Foo for Bar {}

fn main() {
    let bar = Bar {};
    let boxed: Box<dyn Foo> = Box::new(bar);
    // tada:
    let _clone: Box<dyn Foo> = boxed.clone();
}
```

## `PartialEq` but object-safe

Using the `dyn_partial_eq` macro allows you to make your trait equality testable with a single line of code:

```rust
pub trait Foo: FooPartialEq {}

minecrevy_dyn::dyn_partial_eq!(Foo: FooPartialEq);

#[derive(PartialEq)] // Without this, we can't implement Foo!
pub struct Bar(bool);

impl Foo for Bar {}

fn main() {
    let bar1: Box<dyn Foo> = Box::new(Bar(true));
    let bar2: Box<dyn Foo> = Box::new(Bar(false));
    assert_ne!(bar1, bar2);
}
```
