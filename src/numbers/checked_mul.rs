pub trait CheckedMul<Other = Self> {
    type Output;

    /// Returns checked multiplication.
    /// ```
    /// use traiter::numbers::CheckedMul;
    /// // signed integers
    /// assert_eq!(CheckedMul::checked_mul(i8::MIN, -1i8), None);
    /// assert_eq!(CheckedMul::checked_mul(1i8, 2i8), Some(2i8));
    /// assert_eq!(CheckedMul::checked_mul(i8::MAX, 2i8), None);
    /// // unsigned integers
    /// assert_eq!(CheckedMul::checked_mul(0u8, 2u8), Some(0u8));
    /// assert_eq!(CheckedMul::checked_mul(1u8, 2u8), Some(2u8));
    /// assert_eq!(CheckedMul::checked_mul(u8::MAX, 2u8), None);
    /// ```
    fn checked_mul(self, other: Other) -> Self::Output;
}

macro_rules! integer_checked_mul_impl {
    ($($integer:ty)*) => ($(
        impl CheckedMul for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_mul(self, other: Self) -> Self::Output {
                <$integer>::checked_mul(self, other)
            }
        }
    )*)
}

integer_checked_mul_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
