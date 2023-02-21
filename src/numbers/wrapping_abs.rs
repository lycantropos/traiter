pub trait WrappingAbs {
    type Output;

    /// Returns wrapping absolute value.
    /// ```
    /// use traiter::numbers::WrappingAbs;
    /// // signed integers
    /// assert_eq!(WrappingAbs::wrapping_abs(i8::MIN), i8::MIN);
    /// assert_eq!(WrappingAbs::wrapping_abs(0i8), 0i8);
    /// assert_eq!(WrappingAbs::wrapping_abs(i8::MAX), i8::MAX);
    /// ```
    fn wrapping_abs(self) -> Self::Output;
}

macro_rules! integer_wrapping_abs_impl {
    ($($integer:ty)*) => ($(
        impl WrappingAbs for $integer {
            type Output = Self;

            #[inline(always)]
            fn wrapping_abs(self) -> Self::Output {
                <$integer>::wrapping_abs(self)
            }
        }
    )*)
}

integer_wrapping_abs_impl!(i8 i16 i32 i64 i128 isize);
