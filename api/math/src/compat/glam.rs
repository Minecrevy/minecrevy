use crate::vector::{Vector2, Vector3, Vector4};

impl From<glam::IVec2> for Vector2<i32> {
    #[inline]
    fn from(vec: glam::IVec2) -> Self {
        Vector2::new([vec.x, vec.y])
    }
}

impl From<glam::UVec2> for Vector2<u32> {
    #[inline]
    fn from(vec: glam::UVec2) -> Self {
        Vector2::new([vec.x, vec.y])
    }
}

impl From<glam::Vec2> for Vector2<f32> {
    #[inline]
    fn from(vec: glam::Vec2) -> Self {
        Vector2::new([vec.x, vec.y])
    }
}

impl From<glam::DVec2> for Vector2<f64> {
    #[inline]
    fn from(vec: glam::DVec2) -> Self {
        Vector2::new([vec.x, vec.y])
    }
}

impl From<glam::IVec3> for Vector3<i32> {
    #[inline]
    fn from(vec: glam::IVec3) -> Self {
        Vector3::new([vec.x, vec.y, vec.z])
    }
}

impl From<glam::UVec3> for Vector3<u32> {
    #[inline]
    fn from(vec: glam::UVec3) -> Self {
        Vector3::new([vec.x, vec.y, vec.z])
    }
}

impl From<glam::Vec3> for Vector3<f32> {
    #[inline]
    fn from(vec: glam::Vec3) -> Self {
        Vector3::new([vec.x, vec.y, vec.z])
    }
}

impl From<glam::DVec3> for Vector3<f64> {
    #[inline]
    fn from(vec: glam::DVec3) -> Self {
        Vector3::new([vec.x, vec.y, vec.z])
    }
}

impl From<glam::IVec4> for Vector4<i32> {
    #[inline]
    fn from(vec: glam::IVec4) -> Self {
        Vector4::new([vec.x, vec.y, vec.z, vec.w])
    }
}

impl From<glam::UVec4> for Vector4<u32> {
    #[inline]
    fn from(vec: glam::UVec4) -> Self {
        Vector4::new([vec.x, vec.y, vec.z, vec.w])
    }
}

impl From<glam::Vec4> for Vector4<f32> {
    #[inline]
    fn from(vec: glam::Vec4) -> Self {
        Vector4::new([vec.x, vec.y, vec.z, vec.w])
    }
}

impl From<glam::DVec4> for Vector4<f64> {
    #[inline]
    fn from(vec: glam::DVec4) -> Self {
        Vector4::new([vec.x, vec.y, vec.z, vec.w])
    }
}

impl From<Vector2<i32>> for glam::IVec2 {
    #[inline]
    fn from(vec: Vector2<i32>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector2<u32>> for glam::UVec2 {
    #[inline]
    fn from(vec: Vector2<u32>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector2<f32>> for glam::Vec2 {
    #[inline]
    fn from(vec: Vector2<f32>) -> Self {
        Self::from(vec.0)
    }
}
impl From<Vector2<f64>> for glam::DVec2 {
    #[inline]
    fn from(vec: Vector2<f64>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector3<i32>> for glam::IVec3 {
    #[inline]
    fn from(vec: Vector3<i32>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector3<u32>> for glam::UVec3 {
    #[inline]
    fn from(vec: Vector3<u32>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector3<f32>> for glam::Vec3 {
    #[inline]
    fn from(vec: Vector3<f32>) -> Self {
        Self::from(vec.0)
    }
}
impl From<Vector3<f64>> for glam::DVec3 {
    #[inline]
    fn from(vec: Vector3<f64>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector4<i32>> for glam::IVec4 {
    #[inline]
    fn from(vec: Vector4<i32>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector4<u32>> for glam::UVec4 {
    #[inline]
    fn from(vec: Vector4<u32>) -> Self {
        Self::from(vec.0)
    }
}

impl From<Vector4<f32>> for glam::Vec4 {
    #[inline]
    fn from(vec: Vector4<f32>) -> Self {
        Self::from(vec.0)
    }
}
impl From<Vector4<f64>> for glam::DVec4 {
    #[inline]
    fn from(vec: Vector4<f64>) -> Self {
        Self::from(vec.0)
    }
}
