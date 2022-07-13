use super::div_rem_euclid::DivRemEuclid;

pub trait CheckedRemEuclidInv<Divisor = Self> {
    type Output;

    /// Returns multiplicative inverse of euclidean division.
    /// ```
    /// use traiter::numbers::CheckedRemEuclidInv;
    /// // signed integers
    /// assert_eq!(
    ///     CheckedRemEuclidInv::checked_rem_euclid_inv(-3i8, 2i8), Some(1i8)
    /// );
    /// assert_eq!(
    ///     CheckedRemEuclidInv::checked_rem_euclid_inv(-3i8, 1i8), Some(0i8)
    /// );
    /// assert_eq!(
    ///     CheckedRemEuclidInv::checked_rem_euclid_inv(-3i8, 0i8), None
    /// );
    /// // unsigned integers
    /// assert_eq!(
    ///     CheckedRemEuclidInv::checked_rem_euclid_inv(3u8, 2u8), Some(1u8)
    /// );
    /// assert_eq!(
    ///     CheckedRemEuclidInv::checked_rem_euclid_inv(3u8, 1u8), Some(0u8)
    /// );
    /// assert_eq!(
    ///     CheckedRemEuclidInv::checked_rem_euclid_inv(3u8, 0u8), None
    /// );
    /// ```
    fn checked_rem_euclid_inv(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_signed_checked_rem_euclid_inv_impl {
    ($($integer:ty)*) => ($(
        impl CheckedRemEuclidInv for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
                let mut candidate = 0;
                let mut result = 1;
                let mut step_dividend = self;
                let mut step_divisor = divisor;
                while step_divisor != 0 {
                    let (quotient, remainder) =
                        step_dividend.div_rem_euclid(step_divisor);
                    step_dividend = step_divisor;
                    step_divisor = remainder;
                    (result, candidate) =
                        (candidate, result - quotient * candidate);
                }
                if step_dividend == 1 {
                    Some(if result.is_negative() {
                        divisor + result
                    } else {
                        result
                    })
                } else {
                    None
                }
            }
        }
    )*)
}

primitive_signed_checked_rem_euclid_inv_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! primitive_unsigned_checked_rem_euclid_inv_impl {
    ($($integer:ty)*) => ($(
        impl CheckedRemEuclidInv for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_rem_euclid_inv(self, divisor: Self) -> Self::Output {
                let mut candidate_modulus = 0;
                let mut result_modulus = 1;
                let mut is_result_negative = false;
                let mut is_candidate_negative = false;
                let mut step_dividend = self;
                let mut step_divisor = divisor;
                while step_divisor != 0 {
                    let (quotient, remainder) =
                        step_dividend.div_rem_euclid(step_divisor);
                    step_dividend = step_divisor;
                    step_divisor = remainder;
                    let subtrahend_modulus = quotient * candidate_modulus;
                    (
                        is_result_negative,
                        result_modulus,
                        (is_candidate_negative, candidate_modulus),
                    ) = (
                        is_candidate_negative,
                        candidate_modulus,
                        if is_result_negative {
                            if is_candidate_negative {
                                if result_modulus > subtrahend_modulus {
                                    (true, result_modulus - subtrahend_modulus)
                                } else {
                                    (
                                        false,
                                        subtrahend_modulus - result_modulus,
                                    )
                                }
                            } else {
                                (true, subtrahend_modulus + result_modulus)
                            }
                        } else if is_candidate_negative {
                            (false, subtrahend_modulus + result_modulus)
                        } else if result_modulus > subtrahend_modulus {
                            (false, result_modulus - subtrahend_modulus)
                        } else {
                            (true, subtrahend_modulus - result_modulus)
                        },
                    );
                }
                if step_dividend == 1 {
                    Some(if is_result_negative {
                        divisor - result_modulus
                    } else {
                        result_modulus
                    })
                } else {
                    None
                }
            }
        }
    )*)
}

primitive_unsigned_checked_rem_euclid_inv_impl!(u8 u16 u32 u64 u128 usize);
