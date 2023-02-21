pub trait WrappingAdd<Other = Self> {
    type Output;

    /// Returns wrapping addition.
    /// ```
    /// use traiter::numbers::WrappingAdd;
    /// // signed integers
    /// assert_eq!(WrappingAdd::wrapping_add(i8::MIN, -1i8), i8::MAX);
    /// assert_eq!(WrappingAdd::wrapping_add(0i8, 1i8), 1i8);
    /// assert_eq!(WrappingAdd::wrapping_add(i8::MAX, 1i8), i8::MIN);
    /// // unsigned integers
    /// assert_eq!(WrappingAdd::wrapping_add(0u8, 1u8), 1u8);
    /// assert_eq!(WrappingAdd::wrapping_add(1u8, 1u8), 2u8);
    /// assert_eq!(WrappingAdd::wrapping_add(u8::MAX, 1u8), u8::MIN);
    /// ```
    fn wrapping_add(self, other: Other) -> Self::Output;
}

macro_rules! integer_wrapping_add_impl {
    ($($integer:ty)*) => ($(
        impl WrappingAdd for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_add(self, other: Self) -> Self::Output {
                <$integer>::wrapping_add(self, other)
            }
        }
    )*)
}

integer_wrapping_add_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
