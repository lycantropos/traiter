pub trait Trunc {
    type Output;

    /// Returns truncation of a number.
    ///
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::numbers::Trunc;
// floating point numbers
assert_eq!(Trunc::trunc(-1.1_f32), -1.0_f32);
assert_eq!(Trunc::trunc(0.0_f32), 0.0_f32);
assert_eq!(Trunc::trunc(1.1_f32), 1.0_f32);
```
"##
    )]
    fn trunc(self) -> Self::Output;
}

macro_rules! primitive_trunc_impl {
    ($($float:ty)*) => ($(
        impl Trunc for $float {
            type Output = Self;

            #[inline(always)]
            fn trunc(self) -> Self::Output {
                <$float>::trunc(self)
            }
        }
    )*)
}

#[cfg(feature = "std")]
primitive_trunc_impl!(f32 f64);
