use super::types::Sign;
use super::zeroable::Zeroable;
use core::cmp::Ordering;

pub trait Signed: Zeroable {
    /// Checks if a number is negative.
    /// ```
    /// use traiter::numbers::Signed;
    /// // signed integers
    /// assert!(Signed::is_negative(&-1i8));
    /// assert!(!Signed::is_negative(&0i8));
    /// assert!(!Signed::is_negative(&1i8));
    /// ```
    fn is_negative(&self) -> bool;

    /// Checks if a number is positive.
    /// ```
    /// use traiter::numbers::Signed;
    /// // signed integers
    /// assert!(!Signed::is_positive(&-1i8));
    /// assert!(!Signed::is_positive(&0i8));
    /// assert!(Signed::is_positive(&1i8));
    /// ```
    fn is_positive(&self) -> bool;

    /// Returns sign of a number.
    /// ```
    /// use traiter::numbers::{Signed, Sign};
    /// // signed integers
    /// assert_eq!(Signed::sign(&-1i8), Sign::Negative);
    /// assert_eq!(Signed::sign(&0i8), Sign::Zero);
    /// assert_eq!(Signed::sign(&1i8), Sign::Positive);
    /// ```
    fn sign(&self) -> Sign;
}

macro_rules! primitive_signed_impl {
    ($($t:ty)*) => ($(
        impl Signed for $t {
            #[inline(always)]
            fn is_negative(&self) -> bool {
                <$t>::is_negative(*self)
            }

            #[inline(always)]
            fn is_positive(&self) -> bool {
                <$t>::is_positive(*self)
            }

            #[inline(always)]
            fn sign(&self) -> Sign {
                match self.cmp(&(0 as $t)) {
                    Ordering::Equal => Sign::Zero,
                    Ordering::Greater => Sign::Positive,
                    Ordering::Less => Sign::Negative,
                }
            }
        }
    )*)
}

primitive_signed_impl!(i8 i16 i32 i64 i128 isize);
