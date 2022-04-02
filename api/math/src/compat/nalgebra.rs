use crate::vector::{Vector2, Vector3, Vector4};

impl<T> From<nalgebra::Vector2<T>> for Vector2<T> {
    #[inline]
    fn from(vec: nalgebra::Vector2<T>) -> Self {
        let [xy] = vec.data.0;
        Self::new(xy)
    }
}

impl<T> From<nalgebra::Vector3<T>> for Vector3<T> {
    #[inline]
    fn from(vec: nalgebra::Vector3<T>) -> Self {
        let [xyz] = vec.data.0;
        Self::new(xyz)
    }
}

impl<T> From<nalgebra::Vector4<T>> for Vector4<T> {
    #[inline]
    fn from(vec: nalgebra::Vector4<T>) -> Self {
        let [xyzw] = vec.data.0;
        Self::new(xyzw)
    }
}

impl<T: nalgebra::Scalar> From<Vector2<T>> for nalgebra::Vector2<T> {
    #[inline]
    fn from(vec: Vector2<T>) -> Self {
        Self::from(vec.0)
    }
}

impl<T: nalgebra::Scalar> From<Vector3<T>> for nalgebra::Vector3<T> {
    #[inline]
    fn from(vec: Vector3<T>) -> Self {
        Self::from(vec.0)
    }
}

impl<T: nalgebra::Scalar> From<Vector4<T>> for nalgebra::Vector4<T> {
    #[inline]
    fn from(vec: Vector4<T>) -> Self {
        Self::from(vec.0)
    }
}
