pub trait Zero: Sized {
    const ZERO: Self;

    fn is_zero(&self) -> bool
    where
        Self: PartialEq
    {
        *self == Self::ZERO
    }
}

pub trait One: Sized {
    const ONE: Self;

    fn is_one(&self) -> bool
    where
        Self: PartialEq
    {
        *self == Self::ONE
    }
}

pub trait Two: Sized {
    const TWO: Self;
}

macro_rules! zero_impl {
    ($($ty:ty => $zero:expr),+) => {$(
        impl Zero for $ty {
            const ZERO: Self = $zero;
        }
    )+};
}

zero_impl!(
    u8 => 0,
    u16 => 0,
    u32 => 0,
    u64 => 0,
    u128 => 0,
    usize => 0,
    i8 => 0,
    i16 => 0,
    i32 => 0,
    i64 => 0,
    i128 => 0,
    isize => 0,
    f32 => 0.0,
    f64 => 0.0
);

macro_rules! one_impl {
    ($($ty:ty => $one:expr),+) => {$(
        impl One for $ty {
            const ONE: Self = $one;
        }
    )+};
}

one_impl!(
    u8 => 1,
    u16 => 1,
    u32 => 1,
    u64 => 1,
    u128 => 1,
    usize => 1,
    i8 => 1,
    i16 => 1,
    i32 => 1,
    i64 => 1,
    i128 => 1,
    isize => 1,
    f32 => 1.0,
    f64 => 1.0
);

macro_rules! two_impl {
    ($($ty:ty => $two:expr),+) => {$(
        impl Two for $ty {
            const TWO: Self = $two;
        }
    )+};
}

two_impl!(
    u8 => 2,
    u16 => 2,
    u32 => 2,
    u64 => 2,
    u128 => 2,
    usize => 2,
    i8 => 2,
    i16 => 2,
    i32 => 2,
    i64 => 2,
    i128 => 2,
    isize => 2,
    f32 => 2.0,
    f64 => 2.0
);
