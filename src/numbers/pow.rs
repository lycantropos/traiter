pub trait Pow<Exponent> {
    type Output;

    /// Returns power.
    /// ```
    /// use traiter::numbers::Pow;
    /// // signed integers
    /// assert_eq!(Pow::pow(-3i8, 2u32), 9i8);
    /// assert_eq!(Pow::pow(-3i8, 1u32), -3i8);
    /// assert_eq!(Pow::pow(-3i8, 0u32), 1i8);
    /// // unsigned integers
    /// assert_eq!(Pow::pow(3u8, 2u32), 9u8);
    /// assert_eq!(Pow::pow(3u8, 1u32), 3u8);
    /// assert_eq!(Pow::pow(3u8, 0u32), 1u8);
    /// ```
    fn pow(self, exponent: Exponent) -> Self::Output;
}

macro_rules! integer_pow_impl {
    ($($integer:ty)*) => ($(
        impl Pow<u32> for $integer {
            type Output = Self;

            #[inline(always)]
            fn pow(self, exponent: u32) -> Self::Output {
                <$integer>::pow(self, exponent)
            }
        }
    )*)
}

integer_pow_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
