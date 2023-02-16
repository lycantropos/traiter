pub trait CheckedRem<Divisor = Self> {
    type Output;

    /// Returns remainder of checked division.
    /// ```
    /// use traiter::numbers::CheckedRem;
    /// // signed integers
    /// assert_eq!(CheckedRem::checked_rem(-3i8, 2i8), Some(-1i8));
    /// assert_eq!(CheckedRem::checked_rem(-3i8, 1i8), Some(0i8));
    /// assert_eq!(CheckedRem::checked_rem(-3i8, 0i8), None);
    /// assert_eq!(CheckedRem::checked_rem(i8::MIN, -1i8), None);
    /// // unsigned integers
    /// assert_eq!(CheckedRem::checked_rem(3u8, 2u8), Some(1u8));
    /// assert_eq!(CheckedRem::checked_rem(3u8, 1u8), Some(0u8));
    /// assert_eq!(CheckedRem::checked_rem(3u8, 0u8), None);
    /// ```
    fn checked_rem(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_checked_rem_impl {
    ($($integer:ty)*) => ($(
        impl CheckedRem for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_rem(self, divisor: Self) -> Self::Output {
                <$integer>::checked_rem(self, divisor)
            }
        }
    )*)
}

integer_checked_rem_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
