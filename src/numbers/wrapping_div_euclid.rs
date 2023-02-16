pub trait WrappingDivEuclid<Divisor = Self> {
    type Output;

    /// Returns quotient of wrapping euclidean division.
    /// ```
    /// use traiter::numbers::WrappingDivEuclid;
    /// // signed integers
    /// assert_eq!(WrappingDivEuclid::wrapping_div_euclid(-3i8, 2i8), -2i8);
    /// assert_eq!(WrappingDivEuclid::wrapping_div_euclid(-3i8, 1i8), -3i8);
    /// assert_eq!(
    ///     WrappingDivEuclid::wrapping_div_euclid(i8::MIN, -1i8),
    ///     i8::MIN
    /// );
    /// // unsigned integers
    /// assert_eq!(WrappingDivEuclid::wrapping_div_euclid(3u8, 3u8), 1u8);
    /// assert_eq!(WrappingDivEuclid::wrapping_div_euclid(3u8, 2u8), 1u8);
    /// assert_eq!(WrappingDivEuclid::wrapping_div_euclid(3u8, 1u8), 3u8);
    /// ```
    fn wrapping_div_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_wrapping_div_euclid_impl {
    ($($integer:ty)*) => ($(
        impl WrappingDivEuclid for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_div_euclid(self, divisor: Self) -> Self::Output {
                <$integer>::wrapping_div_euclid(self, divisor)
            }
        }
    )*)
}

integer_wrapping_div_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
