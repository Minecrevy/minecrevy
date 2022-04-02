pub trait Pow<Rhs = Self> {
    type Output;

    /// Returns `self` to the power of `rhs`.
    fn pow(self, rhs: Rhs) -> Self::Output;
}

macro_rules! pow_impl {
    ($(fn pow($ty:ty, $exp:ty) -> $method:expr ;)*) => {$(
        impl Pow<$exp> for $ty {
            type Output = $ty;

            fn pow(self, rhs: $exp) -> Self::Output {
                $method(self, rhs)
            }
        }
    )*};
}

pow_impl!(
    fn pow(u32, u32) -> u32::pow;
    fn pow(u64, u32) -> u64::pow;
    fn pow(i32, u32) -> i32::pow;
    fn pow(i64, u32) -> i64::pow;
    fn pow(f32, f32) -> f32::powf;
    fn pow(f32, i32) -> f32::powi;
    fn pow(f64, f64) -> f64::powf;
    fn pow(f64, i32) -> f64::powi;
);
