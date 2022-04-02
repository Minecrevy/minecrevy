use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::num::{Float, Number, One, Pow, Signed, Zero};
use crate::vector::Vector;

/// A const `2 x 2` matrix.
pub type Matrix2x2<T> = Matrix<2, 2, T>;

/// A const `3 x 3` matrix.
pub type Matrix3x3<T> = Matrix<3, 3, T>;

/// A const `4 x 4` matrix.
pub type Matrix4x4<T> = Matrix<4, 4, T>;

/// A const `ROW` x `COL` matrix.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Matrix<const ROW: usize, const COL: usize, T>(pub Vector<ROW, Vector<COL, T>>);

//////////////////////////////////////////////////
// Inherent Methods
//////////////////////////////////////////////////

impl<const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T> {
    #[inline]
    pub const fn new(matrix: Vector<ROW, Vector<COL, T>>) -> Self {
        Self(matrix)
    }

    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Self(Vector::splat(Vector::splat(v)))
    }

    #[inline]
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Matrix<ROW, COL, R> {
        Matrix(self.0.map(|row| row.map(&mut f)))
    }

    #[inline]
    pub fn map_rows<R>(self, mut f: impl FnMut(Vector<COL, T>) -> R) -> Vector<ROW, R> {
        self.0.map(|row| f(row))
    }

    #[inline]
    pub fn zip<U>(self, other: Matrix<ROW, COL, U>) -> Matrix<ROW, COL, (T, U)> {
        Matrix(self.0.zip(other.0).map(|(t, u)| t.zip(u)))
    }

    #[inline]
    pub fn zip_map<U, R>(
        self,
        other: Matrix<ROW, COL, U>,
        mut f: impl FnMut(T, U) -> R,
    ) -> Matrix<ROW, COL, R> {
        self.zip(other).map(|(a, b)| f(a, b))
    }
}

impl<const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T>
where
    T: Number,
{
    pub fn transpose(self) -> Matrix<COL, ROW, T> {
        let mut result = Matrix::ZERO;
        for row in 0..ROW {
            for col in 0..COL {
                result[(col, row)] = self[(row, col)];
            }
        }
        result
    }

    #[inline]
    pub fn transform(self, vec: Vector<COL, T>) -> Vector<ROW, T> {
        self.map_rows(|row| row.dot(vec))
    }
}

impl<const N: usize, T> Matrix<N, N, T>
where
    T: Number,
{
    pub fn identity() -> Self {
        let mut idx = 0;
        Self(Vector::ZERO.map(|mut row: Vector<N, T>| {
            row[idx] = T::ONE;
            idx += 1;
            row
        }))
    }

    pub fn scaling(vec: Vector<N, T>) -> Self {
        let mut result = Self::ZERO;
        for (idx, v) in vec.0.into_iter().enumerate() {
            result[(idx, idx)] = v;
        }
        result
    }

    pub fn translation(vec: Vector<N, T>) -> Self {
        let mut result = Self::identity();
        for (idx, v) in vec.0.into_iter().enumerate() {
            result[(idx, N - 1)] = v;
        }
        result
    }

    #[inline]
    pub fn scale(self, vec: Vector<N, T>) -> Self {
        Self::scaling(vec) * self
    }
}

impl<const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T>
where
    T: Signed,
{
    #[inline]
    pub fn abs(self) -> Self {
        self.map(Signed::abs)
    }
}

impl<const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T>
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
}

//////////////////////////////////////////////////
// Indexing
//////////////////////////////////////////////////

impl<const ROW: usize, const COL: usize, T> Index<usize> for Matrix<ROW, COL, T> {
    type Output = Vector<COL, T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const ROW: usize, const COL: usize, T> IndexMut<usize> for Matrix<ROW, COL, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const ROW: usize, const COL: usize, T> Index<(usize, usize)> for Matrix<ROW, COL, T> {
    type Output = T;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row][col]
    }
}

impl<const ROW: usize, const COL: usize, T> IndexMut<(usize, usize)> for Matrix<ROW, COL, T> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row][col]
    }
}

//////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////

impl<const ROW: usize, const COL: usize, T> Zero for Matrix<ROW, COL, T>
where
    T: Zero,
{
    const ZERO: Self = Self(Vector::ZERO);
}

impl<const ROW: usize, const COL: usize, T> One for Matrix<ROW, COL, T>
where
    T: One,
{
    const ONE: Self = Self(Vector::ONE);
}

//////////////////////////////////////////////////
// Operators
//////////////////////////////////////////////////

impl<const ROW: usize, const COL: usize, T> Add for Matrix<ROW, COL, T>
where
    T: Add<Output=T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Add::add)
    }
}

impl<const ROW: usize, const COL: usize, T> Sub for Matrix<ROW, COL, T>
where
    T: Sub<Output=T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Sub::sub)
    }
}

impl<const ROW: usize, const COL: usize, T> Mul for Matrix<ROW, COL, T>
where
    T: Mul<Output=T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Mul::mul)
    }
}

impl<const ROW: usize, const COL: usize, T> Div for Matrix<ROW, COL, T>
where
    T: Div<Output=T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Div::div)
    }
}

impl<const ROW: usize, const COL: usize, T> Neg for Matrix<ROW, COL, T>
where
    T: Neg<Output=T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl<const ROW: usize, const COL: usize, T, X> Pow<X> for Matrix<ROW, COL, T>
where
    T: Pow<X, Output=T>,
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

impl<const ROW: usize, const COL: usize, T> From<[[T; COL]; ROW]> for Matrix<ROW, COL, T> {
    fn from(arr: [[T; COL]; ROW]) -> Self {
        Self(Vector::from(arr.map(Vector::from)))
    }
}

impl<const ROW: usize, const COL: usize, T> From<Matrix<ROW, COL, T>> for [[T; COL]; ROW] {
    fn from(matrix: Matrix<ROW, COL, T>) -> Self {
        matrix.map_rows(Vector::into).into()
    }
}
