use std::fmt;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use crate::array::ArrayOps;
use crate::index::{W, X, Y, Z};
use crate::num::{Float, Number, Pow, Signed, Zero};
use crate::vector::{Vector, Vector3};

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Quaternion<T>(pub [T; 4]);

//////////////////////////////////////////////////
// Inherent Methods
//////////////////////////////////////////////////

impl<T> Quaternion<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }

    #[inline]
    pub fn splat(v: T) -> Self
    where
        T: Clone,
    {
        Self([v.clone(), v.clone(), v.clone(), v])
    }

    #[inline]
    pub fn map<R>(self, f: impl FnMut(T) -> R) -> Quaternion<R> {
        Quaternion(self.0.map(f))
    }

    #[inline]
    pub fn zip<U>(self, other: Quaternion<U>) -> Quaternion<(T, U)> {
        Quaternion(self.0.zip(other.0))
    }

    #[inline]
    pub fn zip_map<U, R>(
        self,
        other: Quaternion<U>,
        mut f: impl FnMut(T, U) -> R,
    ) -> Quaternion<R> {
        self.zip(other).map(|(a, b)| f(a, b))
    }

    #[inline]
    pub fn fold<R>(self, initial: R, accumulator: impl FnMut(R, T) -> R) -> R {
        self.0.fold(initial, accumulator)
    }
}

impl<T> Quaternion<T>
where
    T: Number,
{
    #[inline]
    pub const fn identity() -> Self {
        Self([T::ZERO, T::ZERO, T::ZERO, T::ONE])
    }

    #[inline]
    pub fn from_real(w: T) -> Self {
        Self([T::ZERO, T::ZERO, T::ZERO, w])
    }

    #[inline]
    pub fn from_imaginary(x: T, y: T, z: T) -> Self {
        Self([x, y, z, T::ZERO])
    }

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

impl<T> Quaternion<T>
where
    T: Signed,
{
    #[inline]
    pub fn conjugate(self) -> Self {
        let Self([x, y, z, w]) = self;
        Self([-x, -y, -z, w])
    }
}

impl<T> Quaternion<T>
where
    T: Float,
{
    pub fn from_axes_angles_deg(rotation: Vector3<T>) -> Self {
        let Vector([yaw, pitch, roll]) = rotation;

        Self::from_angle_deg_axis(yaw, Vector3::UNIT_Y)
            * Self::from_angle_deg_axis(pitch, Vector3::UNIT_X)
            * Self::from_angle_deg_axis(roll, Vector3::UNIT_Z)
    }

    /// Constructs a new [`Quaternion`] from a rotation vector (x: yaw, y: pitch, z: roll).
    pub fn from_axes_angles_rad(rotation: Vector3<T>) -> Self {
        let Vector([yaw, pitch, roll]) = rotation;

        Self::from_angle_rad_axis(yaw, Vector3::UNIT_Y)
            * Self::from_angle_rad_axis(pitch, Vector3::UNIT_X)
            * Self::from_angle_rad_axis(roll, Vector3::UNIT_Z)
    }

    /// Constructs a new [`Quaternion`] from a rotation angle (in degrees) around an axis vector.
    #[inline]
    pub fn from_angle_deg_axis(angle: T, axis: Vector3<T>) -> Self {
        Self::from_angle_rad_axis(angle.to_radians(), axis)
    }

    /// Constructs a new [`Quaternion`] from a rotation angle (in radians) around an axis vector.
    pub fn from_angle_rad_axis(angle: T, axis: Vector3<T>) -> Self {
        let half_angle = angle / T::TWO;
        let q = half_angle.sin() / axis.length();
        let Vector([x, y, z]) = axis;
        Self([x * q, y * q, z * q, half_angle.cos()])
    }

    #[inline]
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn distance(self, other: Self) -> T {
        self.distance_squared(other).sqrt()
    }

    pub fn rotate(self, vec: Vector3<T>) -> Vector3<T> {
        let length = self.length();
        if length.abs() < T::EPSILON {
            return Vector3::ZERO;
        }

        let Vector([x, y, z]) = vec;
        let Quaternion([nx, ny, nz, nw]) = self / length;
        let [px, py, pz, pw] = [
            nw * x + ny * z - nz * y,
            nw * y + nz * x - nx * z,
            nw * z + nx * y - ny * x,
            -nx * x - ny * y - nz * z,
        ];

        Vector([
            pw * -nx + px * nw - py * nz + pz * ny,
            pw * -ny + py * nw - pz * nx + px * nz,
            pw * -nz + pz * nw - pz * ny + py * nx,
        ])
    }

    #[inline]
    pub fn direction(self) -> Vector3<T> {
        self.rotate(Vector3::FORWARD)
    }

    pub fn axis(self) -> Vector3<T> {
        let Self([x, y, z, w]) = self;
        let q = (T::ONE - w * w).sqrt();
        Vector([x, y, z]) / q
    }

    /// Inverts this quaternion.
    pub fn invert(self) -> Self {
        let length_squared = self.length_squared();
        if length_squared < T::EPSILON {
            Self::ZERO
        } else {
            self.conjugate() / length_squared
        }
    }

    /// Normalizes this quaternion into one of unit length.
    pub fn normalize(self) -> Self {
        let length = self.length();
        if length < T::EPSILON {
            Self::ZERO
        } else {
            self / length
        }
    }

    /// Returns the angles in degrees around the x, y, z axes
    /// that correspond to the rotation represented by this quaternion.
    ///
    /// See [`axes_angles_rad`][`Self::axes_angles_rad`] for more information.
    #[inline]
    pub fn axes_angles_deg(self) -> Vector3<T> {
        self.axes_angles_rad().to_degrees()
    }

    /// Returns the angles in radians around the x, y, z axes
    /// that correspond to the rotation represented by this quaternion.
    ///
    /// The returned vector's components represent the following:
    /// - `x`: yaw angle
    /// - `y`: pitch angle
    /// - `z`: roll angle
    pub fn axes_angles_rad(self) -> Vector3<T> {
        let Self([x, y, z, w]) = self;
        let test: T = w * x - y * z;

        if test.abs() < T::from(0.4999) {
            Vector3::new([
                (T::TWO * test).asin(),
                (T::TWO * (w * y + z * x)).atan2(T::ONE - T::TWO * (x * x + y * y)),
                (T::TWO * (w * z + x * y)).atan2(T::ONE - T::TWO * (x * x + z * z)),
            ])
        } else {
            let signum = test.signum();
            Vector3::new([signum * T::HALF_PI, -signum * T::TWO * z.atan2(w), T::ZERO])
        }
    }
}

//////////////////////////////////////////////////
// Std Traits
//////////////////////////////////////////////////

impl<T> fmt::Debug for Quaternion<T>
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

impl<T> fmt::Display for Quaternion<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<T> Default for Quaternion<T>
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

impl<T> Index<usize> for Quaternion<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Quaternion<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Index<X> for Quaternion<T> {
    type Output = T;

    fn index(&self, _: X) -> &Self::Output {
        &self.0[0]
    }
}

impl<T> IndexMut<X> for Quaternion<T> {
    fn index_mut(&mut self, _: X) -> &mut Self::Output {
        &mut self.0[0]
    }
}

impl<T> Index<Y> for Quaternion<T> {
    type Output = T;

    fn index(&self, _: Y) -> &Self::Output {
        &self.0[1]
    }
}

impl<T> IndexMut<Y> for Quaternion<T> {
    fn index_mut(&mut self, _: Y) -> &mut Self::Output {
        &mut self.0[1]
    }
}

impl<T> Index<Z> for Quaternion<T> {
    type Output = T;

    fn index(&self, _: Z) -> &Self::Output {
        &self.0[2]
    }
}

impl<T> IndexMut<Z> for Quaternion<T> {
    fn index_mut(&mut self, _: Z) -> &mut Self::Output {
        &mut self.0[2]
    }
}

impl<T> Index<W> for Quaternion<T> {
    type Output = T;

    fn index(&self, _: W) -> &Self::Output {
        &self.0[3]
    }
}

impl<T> IndexMut<W> for Quaternion<T> {
    fn index_mut(&mut self, _: W) -> &mut Self::Output {
        &mut self.0[3]
    }
}

//////////////////////////////////////////////////
// Constants
//////////////////////////////////////////////////

impl<T> Zero for Quaternion<T>
where
    T: Zero,
{
    const ZERO: Self = Quaternion([T::ZERO; 4]);
}

//////////////////////////////////////////////////
// Operations
//////////////////////////////////////////////////

impl<T> Add for Quaternion<T>
where
    T: Add<Output=T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Add::add)
    }
}

impl<T> Sub for Quaternion<T>
where
    T: Sub<Output=T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_map(rhs, Sub::sub)
    }
}

impl<T> Mul for Quaternion<T>
where
    T: Number,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Self([sx, sy, sz, sw]) = self;
        let Self([rx, ry, rz, rw]) = rhs;

        Self([
            sw * rx + sx * rw + sy * rz - sz * ry,
            sw * ry + sy * rw + sz * rx - sx * rz,
            sw * rz + sz * rw + sx * ry - sy * rx,
            sw * rw - sx * rx - sy * ry - sz * rz,
        ])
    }
}

impl<T> Div for Quaternion<T>
where
    T: Number,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let length_squared = self.length_squared();
        let Self([sx, sy, sz, sw]) = self;
        let Self([rx, ry, rz, rw]) = rhs;

        Self([
            (sx * rw - sw * rx - sz * ry + sy * rz) / length_squared,
            (sy * rw + sz * rx - sw * ry - sx * rz) / length_squared,
            (sz * rw - sy * rx + sx * ry - sw * rz) / length_squared,
            (sw * rw + sx * rx + sy * ry + sz * rz) / length_squared,
        ])
    }
}

impl<T> Mul<T> for Quaternion<T>
where
    T: Clone + Mul<Output=T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.zip_map(Self::splat(rhs), Mul::mul)
    }
}

impl<T> Div<T> for Quaternion<T>
where
    T: Clone + Div<Output=T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self.zip_map(Self::splat(rhs), Div::div)
    }
}

impl<T> Neg for Quaternion<T>
where
    T: Neg<Output=T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

impl<T, X> Pow<X> for Quaternion<T>
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
