pub trait ArrayOps<const N: usize, T> {
    fn zip_map<F, U, R>(self, other: [U; N], f: F) -> [R; N]
    where
        F: FnMut(T, U) -> R;

    fn fold<F, R>(self, init: R, accumulator: F) -> R
    where
        F: FnMut(R, T) -> R;
}

impl<const N: usize, T> ArrayOps<N, T> for [T; N] {
    #[inline]
    fn zip_map<F, U, R>(self, other: [U; N], mut f: F) -> [R; N]
    where
        F: FnMut(T, U) -> R,
    {
        self.zip(other).map(|(a, b)| f(a, b))
    }

    #[inline]
    fn fold<F, R>(self, init: R, accumulator: F) -> R
    where
        F: FnMut(R, T) -> R,
    {
        self.into_iter().fold(init, accumulator)
    }
}
