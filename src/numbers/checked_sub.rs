pub trait CheckedSub<Subtrahend = Self> {
    type Output;

    /// Returns checked subtraction.
    /// ```
    /// use traiter::numbers::CheckedSub;
    /// // signed integers
    /// assert_eq!(CheckedSub::checked_sub(i8::MIN, 1i8), None);
    /// assert_eq!(CheckedSub::checked_sub(0i8, 1i8), Some(-1i8));
    /// assert_eq!(CheckedSub::checked_sub(i8::MAX, -1i8), None);
    /// // unsigned integers
    /// assert_eq!(CheckedSub::checked_sub(0u8, 1u8), None);
    /// assert_eq!(CheckedSub::checked_sub(1u8, 1u8), Some(0u8));
    /// assert_eq!(CheckedSub::checked_sub(u8::MAX, 1u8), Some(254u8));
    /// ```
    fn checked_sub(self, subtrahend: Subtrahend) -> Self::Output;
}

macro_rules! integer_checked_sub_impl {
    ($($integer:ty)*) => ($(
        impl CheckedSub for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_sub(self, subtrahend: Self) -> Self::Output {
                <$integer>::checked_sub(self, subtrahend)
            }
        }
    )*)
}

integer_checked_sub_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
