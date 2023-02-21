pub trait CheckedAbs {
    type Output;

    /// Returns checked absolute value.
    /// ```
    /// use traiter::numbers::CheckedAbs;
    /// // signed integers
    /// assert_eq!(CheckedAbs::checked_abs(i8::MIN), None);
    /// assert_eq!(CheckedAbs::checked_abs(0i8), Some(0i8));
    /// assert_eq!(CheckedAbs::checked_abs(i8::MAX), Some(i8::MAX));
    /// ```
    fn checked_abs(self) -> Self::Output;
}

macro_rules! integer_checked_abs_impl {
    ($($integer:ty)*) => ($(
        impl CheckedAbs for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_abs(self) -> Self::Output {
                <$integer>::checked_abs(self)
            }
        }
    )*)
}

integer_checked_abs_impl!(i8 i16 i32 i64 i128 isize);
