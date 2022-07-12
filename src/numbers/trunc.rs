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
assert_eq!(Trunc::trunc(-1.1f32), -1f32);
assert_eq!(Trunc::trunc(0f32), 0f32);
assert_eq!(Trunc::trunc(1.1f32), 1f32);
```
"##
    )]
    fn trunc(self) -> Self::Output;
}

macro_rules! primitive_trunc_impl {
    ($($t:ty)*) => ($(
        impl Trunc for $t {
            type Output = $t;

            #[inline(always)]
            fn trunc(self) -> Self::Output {
                <$t>::trunc(self)
            }
        }
    )*)
}

#[cfg(feature = "std")]
primitive_trunc_impl!(f32 f64);