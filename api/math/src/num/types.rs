use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::num::ops::Pow;
use crate::num::{One, Two, Zero};

pub trait Number:
    Sized
    + Copy
    + PartialEq
    + PartialOrd
    + Zero
    + One
    + Two
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}

/// Common functionality of signed numbers.
pub trait Signed: Sized + Number + Neg<Output = Self> {
    /// Returns the absolute value of this number.
    fn abs(self) -> Self;

    /// Returns the sign of this number.
    fn signum(self) -> Self;

    /// Returns `true` if this number is positive (i.e. more than zero).
    #[inline]
    fn is_positive(&self) -> bool
    where
        Self: PartialOrd + Zero,
    {
        *self > Self::ZERO
    }

    /// Returns `true` if this number is negative (i.e. less than zero).
    #[inline]
    fn is_negative(&self) -> bool
    where
        Self: PartialOrd + Zero,
    {
        *self < Self::ZERO
    }
}

pub trait Float: Sized + Number + Signed + Pow<Output = Self> + From<f32> {
    const PI: Self;
    const HALF_PI: Self;
    const EPSILON: Self;

    fn floor(self) -> Self;

    fn ceil(self) -> Self;

    fn round(self) -> Self;

    fn trunc(self) -> Self;

    fn fract(self) -> Self;

    fn sqrt(self) -> Self;

    fn sin(self) -> Self;

    fn cos(self) -> Self;

    fn tan(self) -> Self;

    fn asin(self) -> Self;

    fn acos(self) -> Self;

    fn atan(self) -> Self;

    fn atan2(self, other: Self) -> Self;

    fn sinh(self) -> Self;

    fn cosh(self) -> Self;

    fn tanh(self) -> Self;

    fn asinh(self) -> Self;

    fn acosh(self) -> Self;

    fn atanh(self) -> Self;

    fn to_degrees(self) -> Self;

    fn to_radians(self) -> Self;
}

macro_rules! number_impl {
    ($($ty:ty),*) => {$(
        impl Number for $ty {}
    )*};
}

number_impl!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! signed_impl {
    ($($ty:ty),*) => {$(
        impl Signed for $ty {
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline]
            fn signum(self) -> Self {
                self.signum()
            }
        }
    )*};
}

signed_impl!(i8, i16, i32, i64, i128, isize, f32, f64);

macro_rules! float_impl {
    ($($ty:ident),*) => {$(
        impl Float for $ty {
            const PI: Self = ::std::$ty::consts::PI;
            const HALF_PI: Self = ::std::$ty::consts::FRAC_PI_2;
            const EPSILON: Self = <$ty>::EPSILON;

            #[inline]
            fn floor(self) -> Self {
                self.floor()
            }

            #[inline]
            fn ceil(self) -> Self {
                self.ceil()
            }

            #[inline]
            fn round(self) -> Self {
                self.round()
            }

            #[inline]
            fn trunc(self) -> Self {
                self.trunc()
            }

            #[inline]
            fn fract(self) -> Self {
                self.fract()
            }

            #[inline]
            fn sqrt(self) -> Self {
                self.sqrt()
            }

            #[inline]
            fn sin(self) -> Self {
                self.sin()
            }

            #[inline]
            fn cos(self) -> Self {
                self.cos()
            }

            #[inline]
            fn tan(self) -> Self {
                self.tan()
            }

            #[inline]
            fn asin(self) -> Self {
                self.asin()
            }

            #[inline]
            fn acos(self) -> Self {
                self.acos()
            }

            #[inline]
            fn atan(self) -> Self {
                self.atan()
            }

            #[inline]
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }

            #[inline]
            fn sinh(self) -> Self {
                self.sinh()
            }

            #[inline]
            fn cosh(self) -> Self {
                self.cosh()
            }

            #[inline]
            fn tanh(self) -> Self {
                self.tanh()
            }

            #[inline]
            fn asinh(self) -> Self {
                self.asinh()
            }

            #[inline]
            fn acosh(self) -> Self {
                self.acosh()
            }

            #[inline]
            fn atanh(self) -> Self {
                self.atanh()
            }

            #[inline]
            fn to_degrees(self) -> Self {
                self.to_degrees()
            }

            #[inline]
            fn to_radians(self) -> Self {
                self.to_radians()
            }
        }
    )*};
}

float_impl!(f32, f64);
