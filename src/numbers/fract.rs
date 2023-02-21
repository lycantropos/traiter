pub trait Fract {
    type Output;

    /// Returns fractional part of a number.
    ///
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::numbers::Fract;
// floating point numbers
assert_eq!(Fract::fract(-1.5_f32), -0.5_f32);
assert_eq!(Fract::fract(0.0_f32), 0.0_f32);
assert_eq!(Fract::fract(1.5_f32), 0.5_f32);
```
"##
    )]
    fn fract(self) -> Self::Output;
}

macro_rules! float_fract_impl {
    ($($float:ty)*) => ($(
        impl Fract for $float {
            type Output = Self;

            #[inline(always)]
            fn fract(self) -> Self::Output {
                <$float>::fract(self)
            }
        }
    )*)
}

#[cfg(feature = "std")]
float_fract_impl!(f32 f64);
