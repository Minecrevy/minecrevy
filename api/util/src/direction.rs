use std::fmt;
use std::ops::Sub;

use thiserror::Error;

use minecrevy_math::index::{X, Y, Z};
use minecrevy_math::num::{Float, Number, Signed};
use minecrevy_math::vector::{Vector, Vector2, Vector3};

/// The 4 cardinal directions.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "minecrevy_io_str",
    derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite)
)]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(u8))]
pub enum Direction2d {
    /// South.
    South = 0,
    /// West.
    West = 1,
    /// North.
    North = 2,
    /// East.
    East = 3,
}

impl Direction2d {
    /// Gets the [`Direction2d`] of the [`Axis2d`] along the given [`AxisDirection`].
    pub fn from_axis_with(axis: Axis2d, direction: AxisDirection) -> Option<Self> {
        match direction {
            AxisDirection::Plus => Some(Self::from(axis)),
            AxisDirection::Zero => None,
            AxisDirection::Minus => Some(Self::from(axis).opposite()),
        }
    }

    /// Returns this direction as a unit offset [`Vector2`].
    pub fn offset<T>(&self) -> Vector2<T>
    where
        T: Signed,
    {
        match self {
            Self::South => Vector2::new([T::ZERO, T::ONE]),
            Self::West => Vector2::new([-T::ONE, T::ZERO]),
            Self::North => Vector2::new([T::ZERO, -T::ONE]),
            Self::East => Vector2::new([T::ONE, T::ZERO]),
        }
    }

    /// Returns the direction opposite of this one (i.e. 180 degrees from this direction).
    pub fn opposite(&self) -> Self {
        match self {
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South,
            Self::East => Self::West,
        }
    }

    /// Returns `true` if the given direction is opposite this direction.
    #[inline]
    pub fn is_opposite(&self, direction: Self) -> bool {
        self.opposite() == direction
    }
}

/// Error type for conversion from [`Direction3d`] to [`Direction2d`].
#[derive(Error, Debug)]
#[error("{0} can't be represented as a 2-dimensional direction")]
pub struct TryFromDirection3dError(pub Direction3d);

impl TryFrom<Direction3d> for Direction2d {
    type Error = TryFromDirection3dError;

    fn try_from(value: Direction3d) -> Result<Self, Self::Error> {
        match value {
            Direction3d::Down => Err(TryFromDirection3dError(value)),
            Direction3d::Up => Err(TryFromDirection3dError(value)),
            Direction3d::North => Ok(Self::North),
            Direction3d::South => Ok(Self::South),
            Direction3d::West => Ok(Self::West),
            Direction3d::East => Ok(Self::East),
        }
    }
}

impl From<Axis2d> for Direction2d {
    fn from(axis: Axis2d) -> Self {
        match axis {
            Axis2d::X => Self::East,
            Axis2d::Y => Self::South,
        }
    }
}

impl fmt::Display for Direction2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::South => f.write_str("south"),
            Self::West => f.write_str("west"),
            Self::North => f.write_str("north"),
            Self::East => f.write_str("east"),
        }
    }
}

/// [`Direction2d`] plus `Down` and `Up`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "minecrevy_io_str",
    derive(minecrevy_io_str::McRead, minecrevy_io_str::McWrite)
)]
#[cfg_attr(feature = "minecrevy_io_str", io_repr(u8))]
pub enum Direction3d {
    /// Down.
    Down = 0,
    /// Up.
    Up = 1,
    /// North.
    North = 2,
    /// South.
    South = 3,
    /// West.
    West = 4,
    /// East.
    East = 5,
}

impl Direction3d {
    /// Gets the [`Direction3d`] of the [`Axis3d`] along the given [`AxisDirection`].
    pub fn from_axis_with(axis: Axis3d, direction: AxisDirection) -> Option<Self> {
        match direction {
            AxisDirection::Plus => Some(Self::from(axis)),
            AxisDirection::Zero => None,
            AxisDirection::Minus => Some(Self::from(axis).opposite()),
        }
    }

    /// Returns this direction as a unit offset [`Vector2`].
    pub fn offset<T>(&self) -> Vector3<T>
    where
        T: Signed,
    {
        match self {
            Self::Down => Vector3::new([T::ZERO, -T::ONE, T::ZERO]),
            Self::Up => Vector3::new([T::ZERO, T::ONE, T::ZERO]),
            Self::North => Vector3::new([T::ZERO, T::ZERO, -T::ONE]),
            Self::South => Vector3::new([T::ZERO, T::ZERO, T::ONE]),
            Self::West => Vector3::new([-T::ONE, T::ZERO, T::ZERO]),
            Self::East => Vector3::new([T::ONE, T::ZERO, T::ZERO]),
        }
    }

    /// Returns the direction opposite of this one (i.e. 180 degrees from this direction).
    pub fn opposite(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    /// Returns `true` if the given direction is opposite this direction.
    #[inline]
    pub fn is_opposite(&self, direction: Self) -> bool {
        self.opposite() == direction
    }
}

impl From<Direction2d> for Direction3d {
    fn from(d2d: Direction2d) -> Self {
        match d2d {
            Direction2d::South => Self::South,
            Direction2d::West => Self::West,
            Direction2d::North => Self::North,
            Direction2d::East => Self::East,
        }
    }
}

impl From<Axis3d> for Direction3d {
    fn from(axis: Axis3d) -> Self {
        match axis {
            Axis3d::X => Self::East,
            Axis3d::Y => Self::Up,
            Axis3d::Z => Self::South,
        }
    }
}

impl fmt::Display for Direction3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Down => f.write_str("down"),
            Self::Up => f.write_str("up"),
            Self::South => f.write_str("south"),
            Self::West => f.write_str("west"),
            Self::North => f.write_str("north"),
            Self::East => f.write_str("east"),
        }
    }
}

/// A 2-dimensional cartesian axis.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Axis2d {
    /// The X axis.
    X,
    /// The Y axis.
    Y,
}

impl fmt::Display for Axis2d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => f.write_str("x"),
            Self::Y => f.write_str("y"),
        }
    }
}

impl Axis2d {
    /// Returns the closest direction from the given vector.
    /// If the vector is the zero-vector, this method returns [`X`](Self::X).
    pub fn closest<T>(vec: Vector2<T>) -> Self
    where
        T: Signed,
    {
        let Vector([x, y]) = vec.abs();

        if x < y {
            Self::Y
        } else {
            Self::X
        }
    }

    /// Returns `true` if the given vector is along this axis.
    pub fn is_vector_on_axis<T>(&self, vec: Vector2<T>) -> bool
    where
        T: Float,
    {
        vec.abs()
            .normalize_or_zero()
            .sub(self.to_vector())
            .length_squared()
            .is_zero()
    }

    /// Returns the [`Vector2`] component this axis corresponds to.
    #[inline]
    pub fn component<T>(&self, vec: Vector2<T>) -> T
    where
        T: Number,
    {
        match self {
            Self::X => vec[X],
            Self::Y => vec[Y],
        }
    }

    /// Returns the direction of the [`Vector2`]'s component denoted by this axis.
    pub fn direction<T>(&self, vec: Vector2<T>) -> AxisDirection
    where
        T: Signed,
    {
        let component = self.component(vec);
        if component.is_positive() {
            AxisDirection::Plus
        } else if component.is_negative() {
            AxisDirection::Minus
        } else {
            AxisDirection::Zero
        }
    }

    /// Returns the unit [`Vector2`] backing this axis.
    pub fn to_vector<T>(&self) -> Vector2<T>
    where
        T: Number,
    {
        match self {
            Self::X => Vector2::new([T::ONE, T::ZERO]),
            Self::Y => Vector2::new([T::ZERO, T::ONE]),
        }
    }

    /// Returns the unit [`Vector2`] backing this axis, with the given [`AxisDirection`].
    pub fn to_vector_with<T>(&self, direction: AxisDirection) -> Vector2<T>
    where
        T: Signed,
    {
        self.to_vector::<T>() * direction.signum::<T>()
    }

    /// Returns the next axis in the cycle.
    pub const fn cycle(&self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}

/// A 3-dimensional cartesian axis.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Axis3d {
    /// The X axis.
    X,
    /// The Y axis.
    Y,
    /// The Z axis.
    Z,
}

impl fmt::Display for Axis3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => f.write_str("x"),
            Self::Y => f.write_str("y"),
            Self::Z => f.write_str("z"),
        }
    }
}

impl Axis3d {
    /// Returns the closest direction from the given vector.
    /// If the vector is the zero-vector, this method returns [`X`](Self::X).
    /// If the vector has the same length in x and in z direction, [`X`](Self::X) will be returned.
    pub fn closest<T>(vec: Vector3<T>) -> Self
    where
        T: Signed,
    {
        let Vector([x, y, z]) = vec.abs();

        if x < y {
            if z < y {
                Self::Y
            } else {
                Self::Z
            }
        } else if x < z {
            Self::Z
        } else {
            Self::X
        }
    }

    /// Returns `true` if the given vector is along this axis.
    pub fn is_vector_on_axis<T>(&self, vec: Vector3<T>) -> bool
    where
        T: Float,
    {
        vec.abs()
            .normalize_or_zero()
            .sub(self.to_vector())
            .length_squared()
            .is_zero()
    }

    /// Returns the [`Vector3`] component this axis corresponds to.
    pub fn component<T>(&self, vec: Vector3<T>) -> T
    where
        T: Number,
    {
        match self {
            Self::X => vec[X],
            Self::Y => vec[Y],
            Self::Z => vec[Z],
        }
    }

    /// Returns the direction of the [`Vector3`]'s component denoted by this axis.
    pub fn direction<T>(&self, vec: Vector3<T>) -> AxisDirection
    where
        T: Signed,
    {
        let component = self.component(vec);
        if component.is_positive() {
            AxisDirection::Plus
        } else if component.is_negative() {
            AxisDirection::Minus
        } else {
            AxisDirection::Zero
        }
    }

    /// Returns the unit [`Vector3`] backing this axis.
    pub fn to_vector<T>(&self) -> Vector3<T>
    where
        T: Number,
    {
        match self {
            Self::X => Vector3::new([T::ONE, T::ZERO, T::ZERO]),
            Self::Y => Vector3::new([T::ZERO, T::ONE, T::ZERO]),
            Self::Z => Vector3::new([T::ZERO, T::ZERO, T::ONE]),
        }
    }

    /// Returns the unit [`Vector3`] backing this axis, with the given [`AxisDirection`].
    pub fn to_vector_with<T>(&self, direction: AxisDirection) -> Vector3<T>
    where
        T: Signed,
    {
        self.to_vector::<T>() * direction.signum::<T>()
    }

    /// Returns the next axis in the cycle.
    pub const fn cycle(&self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::X,
        }
    }
}

/// The sign value of an [`Axis2d`] or [`Axis3d`].
pub enum AxisDirection {
    /// Positive direction: +1
    Plus,
    /// No direction: 0
    Zero,
    /// Negative direction: -1
    Minus,
}

impl AxisDirection {
    /// Returns the signum for this direction.
    pub fn signum<T>(&self) -> T
    where
        T: Signed,
    {
        match self {
            Self::Plus => T::ONE,
            Self::Zero => T::ZERO,
            Self::Minus => -T::ONE,
        }
    }
}
