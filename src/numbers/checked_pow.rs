pub trait CheckedPow<Exponent> {
    type Output;

    /// Returns checked power.
    /// ```
    /// use traiter::numbers::CheckedPow;
    /// // signed integers
    /// assert_eq!(CheckedPow::checked_pow(-3i8, 2u32), Some(9i8));
    /// assert_eq!(CheckedPow::checked_pow(-3i8, 1u32), Some(-3i8));
    /// assert_eq!(CheckedPow::checked_pow(i8::MAX, 2u32), None);
    /// // unsigned integers
    /// assert_eq!(CheckedPow::checked_pow(3u8, 2u32), Some(9u8));
    /// assert_eq!(CheckedPow::checked_pow(3u8, 1u32), Some(3u8));
    /// assert_eq!(CheckedPow::checked_pow(u8::MAX, 2u32), None);
    /// ```
    fn checked_pow(self, exponent: Exponent) -> Self::Output;
}

macro_rules! primitive_checked_pow_impl {
    ($($integer:ty)*) => ($(
        impl CheckedPow<u32> for $integer {
            type Output = Option<$integer>;

            #[inline(always)]
            fn checked_pow(self, exponent: u32) -> Self::Output {
                <$integer>::checked_pow(self, exponent)
            }
        }
    )*)
}

primitive_checked_pow_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
