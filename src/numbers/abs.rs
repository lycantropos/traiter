/// Absolute value operator.
///
/// ```
/// use traiter::numbers::Abs;
/// // signed integers
/// assert_eq!(Abs::abs(-1), 1);
/// assert_eq!(Abs::abs(0), 0);
/// assert_eq!(Abs::abs(1), 1);
/// // floating point numbers
/// assert_eq!(Abs::abs(-1.), 1.);
/// assert_eq!(Abs::abs(0.), 0.);
/// assert_eq!(Abs::abs(1.), 1.);
/// ```
pub trait Abs {
    type Output;

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

primitive_abs_impl!(f32 f64 i8 i16 i32 i64 i128 isize);
