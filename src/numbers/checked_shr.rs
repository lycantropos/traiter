pub trait CheckedShr<Shift = Self> {
    type Output;

    /// Returns checked shift-right.
    /// ```
    /// use traiter::numbers::CheckedShr;
    /// // signed integers
    /// assert_eq!(CheckedShr::checked_shr(-3i8, 0u32), Some(-3i8));
    /// assert_eq!(CheckedShr::checked_shr(-3i8, 1u32), Some(-2i8));
    /// assert_eq!(CheckedShr::checked_shr(-3i8, 2u32), Some(-1i8));
    /// // unsigned integers
    /// assert_eq!(CheckedShr::checked_shr(3u8, 0u32), Some(3u8));
    /// assert_eq!(CheckedShr::checked_shr(3u8, 1u32), Some(1u8));
    /// assert_eq!(CheckedShr::checked_shr(3u8, 2u32), Some(0u8));
    /// ```
    fn checked_shr(self, shift: Shift) -> Self::Output;
}

macro_rules! integer_checked_shr_impl {
    ($($integer:ty)*) => ($(
        impl CheckedShr<u32> for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_shr(self, shift: u32) -> Self::Output {
                <$integer>::checked_shr(self, shift)
            }
        }
    )*)
}

integer_checked_shr_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
