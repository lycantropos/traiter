use core::ops::{Div, Rem};

pub trait DivRem<Divisor = Self> {
    type Output;

    /// Returns quotient and remainder of division.
    /// ```
    /// use traiter::numbers::DivRem;
    /// // signed integers
    /// assert_eq!(DivRem::div_rem(-3i8, 3i8), (-1i8, 0i8));
    /// assert_eq!(DivRem::div_rem(-3i8, 2i8), (-1i8, -1i8));
    /// assert_eq!(DivRem::div_rem(-3i8, 1i8), (-3i8, 0i8));
    /// // unsigned integers
    /// assert_eq!(DivRem::div_rem(3u8, 3u8), (1u8, 0u8));
    /// assert_eq!(DivRem::div_rem(3u8, 2u8), (1u8, 1u8));
    /// assert_eq!(DivRem::div_rem(3u8, 1u8), (3u8, 0u8));
    /// ```
    fn div_rem(self, divisor: Divisor) -> Self::Output;
}

macro_rules! integer_div_rem_impl {
    ($($integer:ty)*) => ($(
        impl DivRem for $integer {
            type Output = (Self, Self);

            #[inline(always)]
            fn div_rem(self, divisor: Self) -> Self::Output {
                (
                    <$integer>::div(self, divisor),
                    <$integer>::rem(self, divisor),
                )
            }
        }
    )*)
}

integer_div_rem_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
