pub trait CheckedDivEuclid<Divisor = Self> {
    type Output;

    /// Returns quotient of checked euclidean division.
    /// ```
    /// use traiter::numbers::CheckedDivEuclid;
    /// // signed integers
    /// assert_eq!(
    ///     CheckedDivEuclid::checked_div_euclid(-3i8, 2i8), Some(-2i8)
    /// );
    /// assert_eq!(
    ///     CheckedDivEuclid::checked_div_euclid(-3i8, 1i8), Some(-3i8)
    /// );
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(-3i8, 0i8), None);
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(i8::MIN, -1i8), None);
    /// // unsigned integers
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(3u8, 2u8), Some(1u8));
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(3u8, 1u8), Some(3u8));
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(3u8, 0u8), None);
    /// ```
    fn checked_div_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_checked_div_euclid_impl {
    ($($integer:ty)*) => ($(
        impl CheckedDivEuclid for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_div_euclid(self, divisor: Self) -> Self::Output {
                <$integer>::checked_div_euclid(self, divisor)
            }
        }
    )*)
}

integer_checked_div_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
