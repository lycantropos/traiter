pub trait CheckedShl<Shift = Self> {
    type Output;

    /// Returns checked shift-left.
    /// ```
    /// use traiter::numbers::CheckedShl;
    /// // signed integers
    /// assert_eq!(CheckedShl::checked_shl(-3i8, 0u32), Some(-3i8));
    /// assert_eq!(CheckedShl::checked_shl(-3i8, 1u32), Some(-6i8));
    /// assert_eq!(CheckedShl::checked_shl(-3i8, 2u32), Some(-12i8));
    /// // unsigned integers
    /// assert_eq!(CheckedShl::checked_shl(3u8, 0u32), Some(3u8));
    /// assert_eq!(CheckedShl::checked_shl(3u8, 1u32), Some(6u8));
    /// assert_eq!(CheckedShl::checked_shl(3u8, 2u32), Some(12u8));
    /// ```
    fn checked_shl(self, shift: Shift) -> Self::Output;
}

macro_rules! integer_checked_shl_impl {
    ($($integer:ty)*) => ($(
        impl CheckedShl<u32> for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_shl(self, shift: u32) -> Self::Output {
                <$integer>::checked_shl(self, shift)
            }
        }
    )*)
}

integer_checked_shl_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
