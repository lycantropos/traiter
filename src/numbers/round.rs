use super::types::TieBreaking;

pub trait Round {
    type Output;

    /// Returns rounding of a number.
    ///
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
// floating point numbers
use traiter::numbers::{Round, TieBreaking};
assert_eq!(Round::round(-0.5_f32, TieBreaking::AwayFromZero), -1.0_f32);
assert_eq!(Round::round(-0.5_f32, TieBreaking::ToEven), 0.0_f32);
assert_eq!(Round::round(-0.5_f32, TieBreaking::ToOdd), -1.0_f32);
assert_eq!(Round::round(-0.5_f32, TieBreaking::TowardZero), 0.0_f32);
```
"##
    )]
    fn round(self, tie_breaking: TieBreaking) -> Self::Output;
}

macro_rules! primitive_round_impl {
    ($($float:ty)*) => ($(
        impl Round for $float {
            type Output = Self;

            #[inline(always)]
            fn round(self, tie_breaking: TieBreaking) -> Self::Output {
                match tie_breaking {
                    TieBreaking::AwayFromZero => <$float>::round(self),
                    TieBreaking::ToEven => {
                        if self.ceil() - self == (0.5 as $float) {
                            2.0 * <$float>::round(self / 2.0)
                        } else {
                            <$float>::round(self)
                        }
                    }
                    TieBreaking::ToOdd => {
                        if self.ceil() - self == (0.5 as $float) {
                            2.0 * (self / 2.0).floor() + 1.0
                        } else {
                            <$float>::round(self)
                        }
                    }
                    TieBreaking::TowardZero => {
                        if self.ceil() - self == (0.5 as $float) {
                            self.trunc()
                        } else {
                            <$float>::round(self)
                        }
                    }
                }
            }
        }
    )*)
}

#[cfg(feature = "std")]
primitive_round_impl!(f32 f64);
