pub trait CheckedDivEuclid<Divisor = Self> {
    type Output;

    /// Returns checked quotient of euclidean division.
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
    /// // unsigned integers
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(3u8, 2u8), Some(1u8));
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(3u8, 1u8), Some(3u8));
    /// assert_eq!(CheckedDivEuclid::checked_div_euclid(3u8, 0u8), None);
    /// ```
    fn checked_div_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_checked_div_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedDivEuclid for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_div_euclid(self, divisor: Self) -> Self::Output {
                <$t>::checked_div_euclid(self, divisor)
            }
        }
    )*)
}

primitive_checked_div_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
