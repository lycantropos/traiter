pub trait WrappingDiv<Divisor = Self> {
    type Output;

    /// Returns quotient of wrapping division.
    /// ```
    /// use traiter::numbers::WrappingDiv;
    /// // signed integers
    /// assert_eq!(WrappingDiv::wrapping_div(-3i8, 2i8), -1i8);
    /// assert_eq!(WrappingDiv::wrapping_div(-3i8, 1i8), -3i8);
    /// assert_eq!(WrappingDiv::wrapping_div(i8::MIN, -1i8), i8::MIN);
    /// // unsigned integers
    /// assert_eq!(WrappingDiv::wrapping_div(3u8, 3u8), 1u8);
    /// assert_eq!(WrappingDiv::wrapping_div(3u8, 2u8), 1u8);
    /// assert_eq!(WrappingDiv::wrapping_div(3u8, 1u8), 3u8);
    /// ```
    fn wrapping_div(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_wrapping_div_impl {
    ($($integer:ty)*) => ($(
        impl WrappingDiv for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_div(self, divisor: Self) -> Self::Output {
                <$integer>::wrapping_div(self, divisor)
            }
        }
    )*)
}

integer_wrapping_div_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
