pub trait WrappingMul<Other = Self> {
    type Output;

    /// Returns wrapping multiplication.
    /// ```
    /// use traiter::numbers::WrappingMul;
    /// // signed integers
    /// assert_eq!(WrappingMul::wrapping_mul(i8::MIN, -1i8), i8::MIN);
    /// assert_eq!(WrappingMul::wrapping_mul(i8::MAX, -1i8), -i8::MAX);
    /// assert_eq!(WrappingMul::wrapping_mul(i8::MAX, i8::MIN), i8::MIN);
    /// // unsigned integers
    /// assert_eq!(WrappingMul::wrapping_mul(u8::MAX, 2u8), 254u8);
    /// assert_eq!(WrappingMul::wrapping_mul(u8::MAX, 3u8), 253u8);
    /// assert_eq!(WrappingMul::wrapping_mul(u8::MAX, 4u8), 252u8);
    /// ```
    fn wrapping_mul(self, other: Other) -> Self::Output;
}

macro_rules! integer_wrapping_mul_impl {
    ($($integer:ty)*) => ($(
        impl WrappingMul for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_mul(self, other: Self) -> Self::Output {
                <$integer>::wrapping_mul(self, other)
            }
        }
    )*)
}

integer_wrapping_mul_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
