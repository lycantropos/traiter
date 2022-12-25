use core::ops::Rem;

pub trait Gcd<Other = Self> {
    type Output;

    /// Returns greatest common divisor.
    /// ```
    /// use traiter::numbers::Gcd;
    /// // signed integers
    /// assert_eq!(Gcd::gcd(-3i8, 3i8), 3i8);
    /// assert_eq!(Gcd::gcd(-3i8, 2i8), 1i8);
    /// assert_eq!(Gcd::gcd(-3i8, 0i8), 3i8);
    /// // unsigned integers
    /// assert_eq!(Gcd::gcd(3u8, 3u8), 3u8);
    /// assert_eq!(Gcd::gcd(3u8, 2u8), 1u8);
    /// assert_eq!(Gcd::gcd(3u8, 0u8), 3u8);
    /// ```
    fn gcd(self, other: Other) -> Self::Output;
}

macro_rules! signed_integer_gcd_impl {
    ($($integer:ty)*) => ($(
        impl Gcd for $integer {
            type Output = Self;

            #[inline(always)]
            fn gcd(self, other: Self) -> Self::Output {
                let mut first = self.abs();
                let mut second = other.abs();
                while second != 0 {
                    (first, second) = (second, first.rem(second));
                }
                first
            }
        }
    )*)
}

signed_integer_gcd_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_integer_gcd_impl {
    ($($integer:ty)*) => ($(
        impl Gcd for $integer {
            type Output = Self;

            #[inline(always)]
            fn gcd(self, other: Self) -> Self::Output {
                let mut first = self;
                let mut second = other;
                while second != 0 {
                    (first, second) = (second, first.rem(second));
                }
                first
            }
        }
    )*)
}

unsigned_integer_gcd_impl!(u8 u16 u32 u64 u128 usize);
