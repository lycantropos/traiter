pub trait Abs {
    type Output;

    /// Returns absolute value of a number.
    ///
    /// ```
    /// use traiter::numbers::Abs;
    #[cfg_attr(
        feature = "std",
        doc = r##"
// floating point numbers
assert_eq!(Abs::abs(-1.0_f32), 1.0_f32);
assert_eq!(Abs::abs(0.0_f32), 0.0_f32);
assert_eq!(Abs::abs(1.0_f32), 1.0_f32);
"##
    )]
    /// // signed integers
    /// assert_eq!(Abs::abs(-1i8), 1i8);
    /// assert_eq!(Abs::abs(0i8), 0i8);
    /// assert_eq!(Abs::abs(1i8), 1i8);
    /// ```
    fn abs(self) -> Self::Output;
}

macro_rules! number_abs_impl {
    ($($number:ty)*) => ($(
        impl Abs for $number {
            type Output = Self;

            #[inline(always)]
            fn abs(self) -> Self::Output {
                <$number>::abs(self)
            }
        }
    )*)
}

#[cfg(feature = "std")]
number_abs_impl!(f32 f64);
number_abs_impl!(i8 i16 i32 i64 i128 isize);
