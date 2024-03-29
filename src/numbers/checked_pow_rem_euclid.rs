use super::mul_rem::MulRem;

pub trait CheckedPowRemEuclid<Exponent, Divisor> {
    type Output;

    /// Returns remainder of checked power's euclidean division.
    /// ```
    /// use traiter::numbers::CheckedPowRemEuclid;
    /// // signed integers
    /// assert_eq!(
    ///     CheckedPowRemEuclid::checked_pow_rem_euclid(-3i8, 0u32, 2i8),
    ///     Some(1i8)
    /// );
    /// assert_eq!(
    ///     CheckedPowRemEuclid::checked_pow_rem_euclid(-3i8, 1u32, 1i8),
    ///     Some(0i8)
    /// );
    /// assert_eq!(
    ///     CheckedPowRemEuclid::checked_pow_rem_euclid(-3i8, 2u32, 0i8),
    ///     None
    /// );
    /// assert_eq!(
    ///     CheckedPowRemEuclid::checked_pow_rem_euclid(i8::MIN, 1u32, -1i8),
    ///     None
    /// );
    /// // unsigned integers
    /// assert_eq!(
    ///     CheckedPowRemEuclid::checked_pow_rem_euclid(3u8, 0u32, 2u8),
    ///     Some(1u8)
    /// );
    /// assert_eq!(
    ///     CheckedPowRemEuclid::checked_pow_rem_euclid(3u8, 1u32, 1u8),
    ///     Some(0u8)
    /// );
    /// assert_eq!(
    ///     CheckedPowRemEuclid::checked_pow_rem_euclid(3u8, 2u32, 0u8),
    ///     None
    /// );
    /// ```
    fn checked_pow_rem_euclid(
        self,
        exponent: Exponent,
        divisor: Divisor,
    ) -> Self::Output;
}

macro_rules! signed_integer_checked_pow_rem_euclid_impl {
    ($($integer:ty)*) => ($(
        impl CheckedPowRemEuclid<u32, $integer> for $integer {
            type Output = Option<$integer>;

            #[inline]
            fn checked_pow_rem_euclid(
                self,
                exponent: u32,
                divisor: Self,
            ) -> Self::Output {
                let base = if self < 0 || self > divisor {
                    self.checked_rem_euclid(divisor)?
                } else {
                    self
                };
                if divisor == 1 || divisor == -1 {
                    return Some(0);
                }
                let mut result = base;
                let mut exponent_mask = if exponent == 0u32 {
                    0u32
                } else {
                    (1u32 << exponent.ilog2()) >> 1u32
                };
                while exponent_mask != 0 {
                    result = result.mul_rem(result, divisor);
                    if exponent & exponent_mask != 0 {
                        result = result.mul_rem(base, divisor);
                    }
                    exponent_mask >>= 1;
                }
                Some(result)
            }
        }
    )*)
}

signed_integer_checked_pow_rem_euclid_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! unsigned_integer_checked_pow_rem_euclid_impl {
    ($($integer:ty)*) => ($(
        impl CheckedPowRemEuclid<u32, Self> for $integer {
            type Output = Option<Self>;

            #[inline]
            fn checked_pow_rem_euclid(
                self,
                exponent: u32,
                divisor: Self,
            ) -> Self::Output {
                if divisor == 0 {
                    None
                } else if divisor == 1 {
                    Some(0)
                } else {
                    let base = if self > divisor {
                        unsafe {
                            self.checked_rem_euclid(divisor).unwrap_unchecked()
                        }
                    } else {
                        self
                    };
                    let mut result = base;
                    let mut exponent_mask = 2u32;
                    loop {
                        if exponent_mask > exponent {
                            exponent_mask >>= 1;
                            break;
                        }
                        exponent_mask <<= 1;
                    }
                    exponent_mask >>= 1;
                    while exponent_mask != 0 {
                        result = result.mul_rem(result, divisor);
                        if exponent & exponent_mask != 0 {
                            result = result.mul_rem(base, divisor);
                        }
                        exponent_mask >>= 1;
                    }
                    Some(result)
                }
            }
        }
    )*)
}

unsigned_integer_checked_pow_rem_euclid_impl!(u8 u16 u32 u64 u128 usize);
