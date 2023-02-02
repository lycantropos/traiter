pub trait WrappingRemEuclid<Divisor = Self> {
    type Output;

    /// Returns remainder of wrapping euclidean division.
    /// ```
    /// use traiter::numbers::WrappingRemEuclid;
    /// // signed integers
    /// assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(-3i8, 2i8), 1i8);
    /// assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(-3i8, 1i8), 0i8);
    /// assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(i8::MIN, -1i8), 0i8);
    /// // unsigned integers
    /// assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(3u8, 2u8), 1u8);
    /// assert_eq!(WrappingRemEuclid::wrapping_rem_euclid(3u8, 1u8), 0u8);
    /// ```
    fn wrapping_rem_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_wrapping_rem_euclid_impl {
    ($($integer:ty)*) => ($(
        impl WrappingRemEuclid for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_rem_euclid(self, divisor: Self) -> Self::Output {
                <$integer>::wrapping_rem_euclid(self, divisor)
            }
        }
    )*)
}

integer_wrapping_rem_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
