pub trait Floor {
    type Output;

    /// Returns floor of a number.
    ///
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::numbers::Floor;
// floating point numbers
assert_eq!(Floor::floor(-0.1), -1.);
assert_eq!(Floor::floor(0.1), 0.);
assert_eq!(Floor::floor(1.1), 1.);
```
"##
    )]
    fn floor(self) -> Self::Output;
}

macro_rules! primitive_floor_impl {
    ($($t:ty)*) => ($(
        impl Floor for $t {
            type Output = $t;

            #[inline(always)]
            fn floor(self) -> Self::Output {
                <$t>::floor(self)
            }
        }
    )*)
}

#[cfg(feature = "std")]
primitive_floor_impl!(f32 f64);