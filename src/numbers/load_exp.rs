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

macro_rules! float_load_exp_impl {
    ($float:ty, $bits:ty) => {
        impl LoadExp<i32> for $float {
            type Output = Self;

            #[inline]
            fn load_exp(mut self, mut exponent: i32) -> Self::Output {
                const EXPONENT_BASE: $bits =
                    (1 << (<$float>::EXPONENT_BITS_COUNT - 1usize)) - 1;
                const EXPONENT_LOWER_BOUND: i32 = <$float>::MIN_EXP - 1;
                const EXPONENT_UPPER_BOUND: i32 = <$float>::MAX_EXP - 1;
                if exponent > EXPONENT_UPPER_BOUND {
                    const SCALE: $float = unsafe {
                        transmute::<$bits, $float>(
                            (((EXPONENT_BASE as i32) + EXPONENT_UPPER_BOUND)
                                as $bits)
                                << <$float>::SIGNIFICAND_BITS_COUNT,
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
                        <$float>::SIGN_BITS_COUNT
                            + <$float>::SIGNIFICAND_BITS_COUNT;
                    const FIRST_MULTIPLIER: $float = unsafe {
                        transmute::<$bits, $float>(
                            (EXPONENT_BASE
                                + (NON_EXPONENT_BITS_COUNT as $bits))
                                << <$float>::SIGNIFICAND_BITS_COUNT,
                        )
                    };
                    const SECOND_MULTIPLIER: $float = unsafe {
                        transmute::<$bits, $float>(
                            (((EXPONENT_BASE as i32) + EXPONENT_LOWER_BOUND)
                                as $bits)
                                << <$float>::SIGNIFICAND_BITS_COUNT,
                        )
                    };
                    const SCALE: $float = FIRST_MULTIPLIER * SECOND_MULTIPLIER;
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
                    (((EXPONENT_BASE as i32) + exponent) as $bits)
                        << <$float>::SIGNIFICAND_BITS_COUNT,
                )
            }
        }
    };
}

float_load_exp_impl!(f32, u32);
float_load_exp_impl!(f64, u64);
