pub trait Ceil {
    type Output;

    /// Returns ceiling of a number.
    ///
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::numbers::Ceil;
// floating point numbers
assert_eq!(Ceil::ceil(-1.1_f32), -1.0_f32);
assert_eq!(Ceil::ceil(0.0_f32), 0.0_f32);
assert_eq!(Ceil::ceil(1.1_f32), 2.0_f32);
```
"##
    )]
    fn ceil(self) -> Self::Output;
}

macro_rules! primitive_ceil_impl {
    ($($t:ty)*) => ($(
        impl Ceil for $t {
            type Output = $t;

            #[inline(always)]
            fn ceil(self) -> Self::Output {
                <$t>::ceil(self)
            }
        }
    )*)
}

#[cfg(feature = "std")]
primitive_ceil_impl!(f32 f64);
