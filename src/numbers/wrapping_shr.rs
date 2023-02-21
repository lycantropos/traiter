pub trait WrappingShr<Other = Self> {
    type Output;

    /// Returns wrapping shift-right.
    /// ```
    /// use traiter::numbers::WrappingShr;
    /// // signed integers
    /// assert_eq!(WrappingShr::wrapping_shr(i8::MIN, 1u32), -64i8);
    /// assert_eq!(WrappingShr::wrapping_shr(1i8, 1u32), 0i8);
    /// assert_eq!(WrappingShr::wrapping_shr(i8::MAX, 1u32), 63i8);
    /// // unsigned integers
    /// assert_eq!(WrappingShr::wrapping_shr(u8::MIN, 1u32), 0u8);
    /// assert_eq!(WrappingShr::wrapping_shr(1u8, 1u32), 0u8);
    /// assert_eq!(WrappingShr::wrapping_shr(u8::MAX, 1u32), 127u8);
    /// ```
    fn wrapping_shr(self, other: Other) -> Self::Output;
}

macro_rules! integer_wrapping_shr_impl {
    ($($integer:ty)*) => ($(
        impl WrappingShr<u32> for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_shr(self, other: u32) -> Self::Output {
                <$integer>::wrapping_shr(self, other)
            }
        }
    )*)
}

integer_wrapping_shr_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
