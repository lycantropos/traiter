pub trait WrappingDivRem<Divisor = Self> {
    type Output;

    /// Returns quotient and remainder of wrapping division.
    /// ```
    /// use traiter::numbers::WrappingDivRem;
    /// // signed integers
    /// assert_eq!(WrappingDivRem::wrapping_div_rem(-3i8, 3i8), (-1i8, 0i8));
    /// assert_eq!(WrappingDivRem::wrapping_div_rem(-3i8, 2i8), (-1i8, -1i8));
    /// assert_eq!(
    ///     WrappingDivRem::wrapping_div_rem(i8::MIN, -1i8),
    ///     (i8::MIN, 0i8)
    /// );
    /// // unsigned integers
    /// assert_eq!(WrappingDivRem::wrapping_div_rem(3u8, 3u8), (1u8, 0u8));
    /// assert_eq!(WrappingDivRem::wrapping_div_rem(3u8, 2u8), (1u8, 1u8));
    /// assert_eq!(WrappingDivRem::wrapping_div_rem(3u8, 1u8), (3u8, 0u8));
    /// ```
    fn wrapping_div_rem(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_wrapping_div_rem_impl {
    ($($integer:ty)*) => ($(
        impl WrappingDivRem for $integer {
            type Output = (Self, Self);

            #[inline(always)]
            fn wrapping_div_rem(self, divisor: Self) -> Self::Output {
                (
                    <$integer>::wrapping_div(self, divisor),
                    <$integer>::wrapping_rem(self, divisor),
                )
            }
        }
    )*)
}

integer_wrapping_div_rem_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
