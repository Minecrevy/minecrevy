use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::array::ArrayOps;
use crate::index::{W, X, Y, Z};
use crate::num::{Float, Number, One, Pow, Signed, Zero};

use thiserror::Error;

/// A const `2`-dimensional vector.
pub type Vector2<T> = Vector<2, T>;

/// A const `3`-dimensional vector.
pub type Vector3<T> = Vector<3, T>;

/// A const `4`-dimensional vector.
pub type Vector4<T> = Vector<4, T>;

/// A const `N`-dimensional vector.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vector<const N: usize, T>(pub [T; N]);

//////////////////////////////////////////////////
// Error Type
//////////////////////////////////////////////////

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("can't {0} zero vector")]
    ZeroVector(&'static str),
}

//////////////////////////////////////////////////
// Inherent Methods
//////////////////////////////////////////////////

impl<const N: usize, T> Vector<N, T> {
    #[inline]
    pub const fn new(components: [T; N]) -> Self {
        Self(components)
    }

    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Self([(); N].map(|()| v.clone()))
    }

    #[inline]
    pub fn map<R>(self, f: impl FnMut(T) -> R) -> Vector<N, R> {
        Vector(self.0.map(f))
    }

    #[inline]
    pub fn zip<U>(self, other: Vector<N, U>) -> Vector<N, (T, U)> {
        Vector(self.0.zip(other.0))
    }

    #[inline]
    pub fn zip_map<U, R>(self, other: Vector<N, U>, mut f: impl FnMut(T, U) -> R) -> Vector<N, R> {
        self.zip(other).map(|(a, b)| f(a, b))
    }

    #[inline]
    pub fn fold<R>(self, initial: R, accumulator: impl FnMut(R, T) -> R) -> R {
        self.0.fold(initial, accumulator)
    }
}

impl<T> Vector<3, T>
where
    T: Number,
{
    pub const UNIT_X: Self = Self([T::ONE, T::ZERO, T::ZERO]);

    pub const UNIT_Y: Self = Self([T::ZERO, T::ONE, T::ZERO]);

    pub const UNIT_Z: Self = Self([T::ZERO, T::ZERO, T::ONE]);

    pub const RIGHT: Self = Self::UNIT_X;

    pub const UP: Self = Self::UNIT_Y;

    pub const FORWARD: Self = Self::UNIT_Z;
}

impl<const N: usize, T> Vector<N, T>
where
    T: Number,
{
    #[inline]
    pub fn dot(self, other: Self) -> T {
        self.zip_map(other, Mul::mul).fold(T::ZERO, Add::add)
    }

    #[inline]
    pub fn length_squared(self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn distance_squared(self, other: Self) -> T {
        self.zip_map(other, Sub::sub).length_squared()
    }
}

impl<const N: usize, T> Vector<N, T>
where
    T: Signed,
{
    #[inline]
    pub fn abs(self) -> Self {
        self.map(Signed::abs)
    }
}

impl<const N: usize, T> Vector<N, T>
where
    T: Float,
{
    #[inline]
    pub fn floor(self) -> Self {
        self.map(Float::floor)
    }

    #[inline]
    pub fn ceil(self) -> Self {
        self.map(Float::ceil)
    }

    #[inline]
    pub fn round(self) -> Self {
        self.map(Float::round)
    }

    #[inline]
    pub fn to_degrees(self) -> Self {
        self.map(Float::to_degrees)
    }

    #[inline]
    pub fn to_radians(self) -> Self {
        self.map(Float::to_radians)
    }

    #[inline]
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn distance(self, other: Self) -> T {
        self.distance_squared(other).sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Result<Self> {
        let len: T = self.length();
        if len.abs() < T::EPSILON {
            Err(Error::ZeroVector("normalize"))
        } else {
            Ok(self / len)
        }
    }

    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        self.normalize().unwrap_or(Vector::ZERO)
    }

    #[inline]
    pub fn project(self, other: Self) -> Result<Self> {
        let length_squared: T = self.length_squared();
        if length_squared < T::EPSILON {
            Err(Error::ZeroVector("project"))
        } else {
            let mult: T = self.clone().dot(other) / length_squared;
            Ok(self * mult)
        }
    }

    #[inline]
    pub fn project_or_zero(self, other: Self) -> Self {
        self.project(other).unwrap_or(Vector::ZERO)
    }
}

//////////////////////////////////////////////////
// Std Traits
//////////////////////////////////////////////////

impl<const N: usize, T> fmt::Debug for Vector<N, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tup = f.debug_tuple("");
        for field in &self.0 {
            tup.field(&field);
        }
        tup.finish()
    }
}

impl<const N: usize, T> fmt::Display for Vector<N, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<const N: usize, T> PartialOrd for Vector<N, T>
where
    T: Number,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length_squared().partial_cmp(&other.length_squared())
    }
}

impl<const N: usize, T> Ord for Vector<N, T>
where
    T: Number + Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.length_squared().cmp(&other.length_squared())
    }
}

impl<const N: usize, T> Default for Vector<N, T>
where
    T: Number,
{
    fn default() -> Self {
        Self::ZERO
    }
}

//////////////////////////////////////////////////
// Indexing
//////////////////////////////////////////////////

impl<const N: usize, T> Index<usize> for Vector<N, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize, T> IndexMut<usize> for Vector<N, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize, T> Index<X> for Vector<N, T> {
    type Output = T;

    fn index(&self, _: X) -> &Self::Output {
        &self.0[0]
    }
}

impl<const N: usize, T> IndexMut<X> for Vector<N, T> {
    fn index_mut(&mut self, _: X) -> &mut Self::Output {
        &mut self.0[0]
    }
}

impl<const N: usize, T> Index<Y> for Vector<N, T> {
    type Output = T;

    fn index(&self, _: Y) -> &Self::Output {
        &self.0[1]
    }
}

impl<const N: usize, T> IndexMut<Y> for Vector<N, T> {
    fn index_mut(&mut self, _: Y) -> &mut Self::Output {
        &mut self.0[1]
    }
}

impl<const N: usize, T> Index<Z> for Vector<N, T> {
    type Output = T;

    fn index(&self, _: Z) -> &Self::Output {
        &self.0[2]
    }
}

impl<const N: usize, T> IndexMut<Z> for Vector<N, T> {
    fn index_mut(&mut self, _: Z) -> &mut Self::Output {
        &mut self.0[2]
    }
}

impl<const N: usize, T> Index<W> for Vector<N, T> {
    type Output = T;

    fn index(&self, _: W) -> &Self::Output {
        &self.0[3]
    }
}

impl<const N: usize, T> IndexMut<W> for Vector<N, T> {
    fn index_mut(&mut self, _: W) -> &mut Self::Output {
        &mut self.0[3]
    }
}

//////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////

impl<const N: usize, T> Zero for Vector<N, T>
where
    T: Zero,
{
    const ZERO: Self = Vector([T::ZERO; N]);
}

impl<const N: usize, T> One for Vector<N, T>
where
    T: One,
{
    const ONE: Self = Vector([T::ONE; N]);
}

//////////////////////////////////////////////////
// Operations
//////////////////////////////////////////////////

impl<const N: usize, T> Add for Vector<N, T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Add::add)
    }
}

impl<const N: usize, T> Sub for Vector<N, T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Sub::sub)
    }
}

impl<const N: usize, T> Mul for Vector<N, T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Mul::mul)
    }
}

impl<const N: usize, T> Div for Vector<N, T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Div::div)
    }
}

impl<const N: usize, T> Mul<T> for Vector<N, T>
where
    T: Clone + Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.zip_map(Self::splat(rhs), Mul::mul)
    }
}

impl<const N: usize, T> Div<T> for Vector<N, T>
where
    T: Clone + Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self.zip_map(Self::splat(rhs), Div::div)
    }
}

impl<const N: usize, T> Neg for Vector<N, T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl<const N: usize, T, X> Pow<X> for Vector<N, T>
where
    T: Pow<X, Output = T>,
    X: Clone,
{
    type Output = Self;

    #[inline]
    fn pow(self, rhs: X) -> Self::Output {
        self.map(|v| v.pow(rhs.clone()))
    }
}

//////////////////////////////////////////////////
// Conversions
//////////////////////////////////////////////////

impl<const N: usize, T> From<Vector<N, T>> for [T; N] {
    #[inline]
    fn from(vec: Vector<N, T>) -> Self {
        vec.0
    }
}

impl<const N: usize, T> From<[T; N]> for Vector<N, T> {
    #[inline]
    fn from(arr: [T; N]) -> Self {
        Self(arr)
    }
}
