/// Ceiling function.
///
#[cfg_attr(
    feature = "std",
    doc = r##"
```
use traiter::numbers::Ceil;
// floating point numbers
assert_eq!(Ceil::ceil(-1.9), -1.);
assert_eq!(Ceil::ceil(0.), 0.);
assert_eq!(Ceil::ceil(0.1), 1.);
```
"##
)]
pub trait Ceil {
    type Output;

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
