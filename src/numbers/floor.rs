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
assert_eq!(Floor::floor(-1.1_f32), -2.0_f32);
assert_eq!(Floor::floor(0.0_f32), 0.0_f32);
assert_eq!(Floor::floor(1.1_f32), 1.0_f32);
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
