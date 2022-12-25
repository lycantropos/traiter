use core::num::ParseIntError;

pub trait FromStrRadix: Sized {
    type Error;

    /// Creates number from a string representation in given base.
    /// ```
    /// use traiter::numbers::FromStrRadix;
    /// // signed integers
    /// assert_eq!(<i8 as FromStrRadix>::from_str_radix(&"-10", 2), Ok(-2i8));
    /// assert_eq!(<i8 as FromStrRadix>::from_str_radix(&"10", 8), Ok(8i8));
    /// assert!(<i8 as FromStrRadix>::from_str_radix(&"", 10).is_err());
    /// // unsigned integers
    /// assert_eq!(<u8 as FromStrRadix>::from_str_radix(&"10", 2), Ok(2u8));
    /// assert_eq!(<u8 as FromStrRadix>::from_str_radix(&"10", 8), Ok(8u8));
    /// assert!(<u8 as FromStrRadix>::from_str_radix(&"", 10).is_err());
    /// ```
    fn from_str_radix(string: &str, radix: u32) -> Result<Self, Self::Error>;
}

macro_rules! integer_from_str_radix_impl {
    ($($integer:ty)*) => ($(
        impl FromStrRadix for $integer {
            type Error = ParseIntError;

            #[inline(always)]
            fn from_str_radix(
                string: &str,
                radix: u32,
            ) -> Result<Self, Self::Error> {
                <$integer>::from_str_radix(string, radix)
            }
        }
    )*)
}

integer_from_str_radix_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
