use core::mem::transmute;

use super::float_info::FloatInfo;

pub trait LoadExp<Exponent> {
    type Output;

    /// Returns multiplication of a number by a power of 2 with given exponent.
    /// ```
    /// use traiter::numbers::LoadExp;
    /// // floating point numbers
    /// assert_eq!(LoadExp::load_exp(0.5_f32, -1i32), 0.25_f32);
    /// assert_eq!(LoadExp::load_exp(0.5_f32, 0i32), 0.5_f32);
    /// assert_eq!(LoadExp::load_exp(0.5_f32, 1i32), 1.0_f32);
    /// ```
    fn load_exp(self, exponent: Exponent) -> Self::Output;
}

macro_rules! primitive_load_exp_impl {
    ($f:ty, $t:ty) => {
        impl LoadExp<i32> for $f {
            type Output = Self;

            fn load_exp(mut self, mut exponent: i32) -> Self::Output {
                const EXPONENT_BASE: $t =
                    (1 << (<$f>::EXPONENT_BITS_COUNT - 1usize)) - 1;
                const EXPONENT_LOWER_BOUND: i32 = <$f>::MIN_EXP - 1;
                const EXPONENT_UPPER_BOUND: i32 = <$f>::MAX_EXP - 1;
                if exponent > EXPONENT_UPPER_BOUND {
                    const SCALE: $f = unsafe {
                        transmute::<$t, $f>(
                            (((EXPONENT_BASE as i32) + EXPONENT_UPPER_BOUND)
                                as $t)
                                << <$f>::SIGNIFICAND_BITS_COUNT,
                        )
                    };
                    self *= SCALE;
                    exponent -= EXPONENT_UPPER_BOUND;
                    if exponent > EXPONENT_UPPER_BOUND {
                        self *= SCALE;
                        exponent -= EXPONENT_UPPER_BOUND;
                        if exponent > EXPONENT_UPPER_BOUND {
                            exponent = EXPONENT_UPPER_BOUND;
                        }
                    }
                } else if exponent < EXPONENT_LOWER_BOUND {
                    const NON_EXPONENT_BITS_COUNT: usize =
                        <$f>::SIGN_BITS_COUNT + <$f>::SIGNIFICAND_BITS_COUNT;
                    const FIRST_MULTIPLIER: $f = unsafe {
                        transmute::<$t, $f>(
                            (EXPONENT_BASE + (NON_EXPONENT_BITS_COUNT as $t))
                                << <$f>::SIGNIFICAND_BITS_COUNT,
                        )
                    };
                    const SECOND_MULTIPLIER: $f = unsafe {
                        transmute::<$t, $f>(
                            (((EXPONENT_BASE as i32) + EXPONENT_LOWER_BOUND)
                                as $t)
                                << <$f>::SIGNIFICAND_BITS_COUNT,
                        )
                    };
                    const SCALE: $f = FIRST_MULTIPLIER * SECOND_MULTIPLIER;
                    self *= SCALE;
                    const EXPONENT_DECREMENT: i32 = (NON_EXPONENT_BITS_COUNT
                        as i32)
                        + EXPONENT_LOWER_BOUND;
                    exponent -= EXPONENT_DECREMENT;
                    if exponent < EXPONENT_LOWER_BOUND {
                        self *= SCALE;
                        exponent -= EXPONENT_DECREMENT;
                        if exponent < EXPONENT_LOWER_BOUND {
                            exponent = EXPONENT_LOWER_BOUND;
                        }
                    }
                }
                self * Self::from_bits(
                    (((EXPONENT_BASE as i32) + exponent) as $t)
                        << <$f>::SIGNIFICAND_BITS_COUNT,
                )
            }
        }
    };
}

primitive_load_exp_impl!(f32, u32);
primitive_load_exp_impl!(f64, u64);
