pub trait DivRemEuclid<Divisor = Self> {
    type Output;

    /// Returns quotient and remainder of euclidean division.
    /// ```
    /// use traiter::numbers::DivRemEuclid;
    /// // signed integers
    /// assert_eq!(DivRemEuclid::div_rem_euclid(-3i8, 3i8), (-1i8, 0i8));
    /// assert_eq!(DivRemEuclid::div_rem_euclid(-3i8, 2i8), (-2i8, 1i8));
    /// assert_eq!(DivRemEuclid::div_rem_euclid(-3i8, 1i8), (-3i8, 0i8));
    /// // unsigned integers
    /// assert_eq!(DivRemEuclid::div_rem_euclid(3u8, 3u8), (1u8, 0u8));
    /// assert_eq!(DivRemEuclid::div_rem_euclid(3u8, 2u8), (1u8, 1u8));
    /// assert_eq!(DivRemEuclid::div_rem_euclid(3u8, 1u8), (3u8, 0u8));
    /// ```
    fn div_rem_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_div_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl DivRemEuclid for $t {
            type Output = (Self, Self);

            #[inline(always)]
            fn div_rem_euclid(self, divisor: Self) -> Self::Output {
                (<$t>::div_euclid(self, divisor), <$t>::rem_euclid(self, divisor))
            }
        }
    )*)
}

primitive_div_rem_euclid_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
