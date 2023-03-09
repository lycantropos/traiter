pub trait WrappingSub<Subtrahend = Self> {
    type Output;

    /// Returns wrapping subtraction.
    /// ```
    /// use traiter::numbers::WrappingSub;
    /// // signed integers
    /// assert_eq!(WrappingSub::wrapping_sub(i8::MIN, 1i8), i8::MAX);
    /// assert_eq!(WrappingSub::wrapping_sub(0i8, 1i8), -1i8);
    /// assert_eq!(WrappingSub::wrapping_sub(i8::MAX, -1i8), i8::MIN);
    /// // unsigned integers
    /// assert_eq!(WrappingSub::wrapping_sub(u8::MIN, 1u8), u8::MAX);
    /// assert_eq!(WrappingSub::wrapping_sub(1u8, 1u8), 0u8);
    /// assert_eq!(WrappingSub::wrapping_sub(u8::MAX, 1u8), 254u8);
    /// ```
    fn wrapping_sub(self, subtrahend: Subtrahend) -> Self::Output;
}

macro_rules! integer_wrapping_sub_impl {
    ($($integer:ty)*) => ($(
        impl WrappingSub for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_sub(self, subtrahend: Self) -> Self::Output {
                <$integer>::wrapping_sub(self, other)
            }
        }
    )*)
}

integer_wrapping_sub_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
