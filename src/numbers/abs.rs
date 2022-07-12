pub trait Abs {
    type Output;

    /// Returns absolute value of a number.
    ///
    /// ```
    /// use traiter::numbers::Abs;
    /// // signed integers
    /// assert_eq!(Abs::abs(-1i8), 1i8);
    /// assert_eq!(Abs::abs(0i8), 0i8);
    /// assert_eq!(Abs::abs(1i8), 1i8);
    #[cfg_attr(
        feature = "std",
        doc = r##"
// floating point numbers
assert_eq!(Abs::abs(-1f32), 1f32);
assert_eq!(Abs::abs(0f32), 0f32);
assert_eq!(Abs::abs(1f32), 1f32);
"##
    )]
    /// ```
    fn abs(self) -> Self::Output;
}

macro_rules! primitive_abs_impl {
    ($($t:ty)*) => ($(
        impl Abs for $t {
            type Output = $t;

            #[inline(always)]
            fn abs(self) -> Self::Output {
                <$t>::abs(self)
            }
        }
    )*)
}

#[cfg(feature = "std")]
primitive_abs_impl!(f32 f64);
primitive_abs_impl!(i8 i16 i32 i64 i128 isize);
