pub trait WrappingRem<Divisor = Self> {
    type Output;

    /// Returns remainder of wrapping division.
    /// ```
    /// use traiter::numbers::WrappingRem;
    /// // signed integers
    /// assert_eq!(WrappingRem::wrapping_rem(-3i8, 2i8), -1i8);
    /// assert_eq!(WrappingRem::wrapping_rem(-3i8, 1i8), 0i8);
    /// assert_eq!(WrappingRem::wrapping_rem(i8::MIN, -1i8), 0i8);
    /// // unsigned integers
    /// assert_eq!(WrappingRem::wrapping_rem(3u8, 2u8), 1u8);
    /// assert_eq!(WrappingRem::wrapping_rem(3u8, 1u8), 0u8);
    /// ```
    fn wrapping_rem(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_wrapping_rem_impl {
    ($($integer:ty)*) => ($(
        impl WrappingRem for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_rem(self, divisor: Self) -> Self::Output {
                <$integer>::wrapping_rem(self, divisor)
            }
        }
    )*)
}

integer_wrapping_rem_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
