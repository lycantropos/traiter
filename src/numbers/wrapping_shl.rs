pub trait WrappingShl<Other = Self> {
    type Output;

    /// Returns wrapping shift-left.
    /// ```
    /// use traiter::numbers::WrappingShl;
    /// // signed integers
    /// assert_eq!(WrappingShl::wrapping_shl(i8::MIN, 1u32), 0i8);
    /// assert_eq!(WrappingShl::wrapping_shl(1i8, 1u32), 2i8);
    /// assert_eq!(WrappingShl::wrapping_shl(i8::MAX, 1u32), -2i8);
    /// // unsigned integers
    /// assert_eq!(WrappingShl::wrapping_shl(u8::MIN, 1u32), 0u8);
    /// assert_eq!(WrappingShl::wrapping_shl(1u8, 1u32), 2u8);
    /// assert_eq!(WrappingShl::wrapping_shl(u8::MAX, 1u32), 254u8);
    /// ```
    fn wrapping_shl(self, other: Other) -> Self::Output;
}

macro_rules! integer_wrapping_shl_impl {
    ($($integer:ty)*) => ($(
        impl WrappingShl<u32> for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_shl(self, other: u32) -> Self::Output {
                <$integer>::wrapping_shl(self, other)
            }
        }
    )*)
}

integer_wrapping_shl_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
