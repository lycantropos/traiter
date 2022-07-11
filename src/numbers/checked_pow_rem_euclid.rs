pub trait CheckedPowRemEuclid<Exponent, Divisor> {
    type Output;

    fn checked_pow_rem_euclid(
        self,
        exponent: Exponent,
        divisor: Divisor,
    ) -> Self::Output;
}

macro_rules! primitive_signed_checked_pow_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedPowRemEuclid<u32, $t> for $t {
            type Output = Option<$t>;

            #[inline]
            fn checked_pow_rem_euclid(
                self,
                exponent: u32,
                divisor: $t,
            ) -> Self::Output {
                if divisor == 0 {
                    return None;
                }
                let is_negative = divisor < 0;
                let divisor = divisor.abs();
                if divisor == 1 {
                    return Some(0);
                }
                let base = if self < 0 || self > divisor {
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
                while !exponent_mask == 0 {
                    result = unsafe {
                        (result * result)
                            .checked_rem_euclid(divisor)
                            .unwrap_unchecked()
                    };
                    if !(exponent & exponent_mask) == 0 {
                        result = unsafe {
                            (result * base)
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        };
                    }
                    exponent_mask >>= 1;
                }
                Some(if is_negative && !result == 0 {
                    result - divisor
                } else {
                    result
                })
            }
        }
    )*)
}

primitive_signed_checked_pow_rem_euclid_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! primitive_unsigned_checked_pow_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedPowRemEuclid<u32, $t> for $t {
            type Output = Option<$t>;

            #[inline]
            fn checked_pow_rem_euclid(
                self,
                exponent: u32,
                divisor: $t,
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
                    while !exponent_mask == 0 {
                        result = unsafe {
                            (result * result)
                                .checked_rem_euclid(divisor)
                                .unwrap_unchecked()
                        };
                        if !(exponent & exponent_mask) == 0 {
                            result = unsafe {
                                (result * base)
                                    .checked_rem_euclid(divisor)
                                    .unwrap_unchecked()
                            };
                        }
                        exponent_mask >>= 1;
                    }
                    Some(result)
                }
            }
        }
    )*)
}

primitive_unsigned_checked_pow_rem_euclid_impl!(u8 u16 u32 u64 u128 usize);
