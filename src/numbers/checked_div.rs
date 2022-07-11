pub trait CheckedDiv<Divisor = Self> {
    type Output;

    /// Returns checked quotient of division.
    /// ```
    /// use traiter::numbers::CheckedDiv;
    /// // signed integers
    /// assert_eq!(CheckedDiv::checked_div(-3i8, 2i8), Some(-1i8));
    /// assert_eq!(CheckedDiv::checked_div(-3i8, 1i8), Some(-3i8));
    /// assert_eq!(CheckedDiv::checked_div(-3i8, 0i8), None);
    /// // unsigned integers
    /// assert_eq!(CheckedDiv::checked_div(3u8, 2u8), Some(1u8));
    /// assert_eq!(CheckedDiv::checked_div(3u8, 1u8), Some(3u8));
    /// assert_eq!(CheckedDiv::checked_div(3u8, 0u8), None);
    /// ```
    fn checked_div(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_checked_div_impl {
    ($($t:ty)*) => ($(
        impl CheckedDiv for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_div(self, divisor: Self) -> Self::Output {
                <$t>::checked_div(self, divisor)
            }
        }
    )*)
}

primitive_checked_div_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
