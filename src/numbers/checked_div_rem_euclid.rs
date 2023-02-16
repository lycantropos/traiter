pub trait CheckedDivRemEuclid<Divisor = Self> {
    type Output;

    /// Returns quotient and remainder of checked euclidean division.
    /// ```
    /// use traiter::numbers::CheckedDivRemEuclid;
    /// // signed integers
    /// assert_eq!(
    ///     CheckedDivRemEuclid::checked_div_rem_euclid(-3i8, 2i8),
    ///     Some((-2i8, 1i8))
    /// );
    /// assert_eq!(
    ///     CheckedDivRemEuclid::checked_div_rem_euclid(-3i8, 1i8),
    ///     Some((-3i8, 0i8))
    /// );
    /// assert_eq!(
    ///     CheckedDivRemEuclid::checked_div_rem_euclid(-3i8, 0i8), None
    /// );
    /// assert_eq!(
    ///     CheckedDivRemEuclid::checked_div_rem_euclid(i8::MIN, -1i8), None
    /// );
    /// // unsigned integers
    /// assert_eq!(
    ///     CheckedDivRemEuclid::checked_div_rem_euclid(3u8, 2u8),
    ///     Some((1u8, 1u8))
    /// );
    /// assert_eq!(
    ///     CheckedDivRemEuclid::checked_div_rem_euclid(3u8, 1u8),
    ///     Some((3u8, 0u8))
    /// );
    /// assert_eq!(
    ///     CheckedDivRemEuclid::checked_div_rem_euclid(3u8, 0u8), None
    /// );
    /// ```
    fn checked_div_rem_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_checked_div_rem_euclid_impl {
    ($($integer:ty)*) => ($(
        impl CheckedDivRemEuclid for $integer {
            type Output = Option<(Self, Self)>;

            #[inline(always)]
            fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
                if divisor == 0 {
                    None
                } else {
                    Some((
                        <$integer>::div_euclid(self, divisor),
                        <$integer>::rem_euclid(self, divisor),
                    ))
                }
            }
        }
    )*)
}

integer_checked_div_rem_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
