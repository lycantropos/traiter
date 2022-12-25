use core::mem::transmute;

use super::float_info::FloatInfo;

pub trait FractExp {
    type Output;

    /// Returns normalized fraction and exponent of a number.
    /// ```
    /// // floating point numbers
    /// use traiter::numbers::FractExp;
    /// assert_eq!(FractExp::fract_exp(-1.5_f32), (-0.75_f32, 1));
    /// assert_eq!(FractExp::fract_exp(0.0_f32), (0.0_f32, 0));
    /// assert_eq!(FractExp::fract_exp(1.5_f32), (0.75_f32, 1));
    /// ```
    fn fract_exp(self) -> Self::Output;
}

macro_rules! float_fract_exp_impl {
    ($float:ty, $bits:ty) => {
        impl FractExp for $float {
            type Output = (Self, i32);

            #[inline]
            fn fract_exp(self) -> Self::Output {
                let bits = self.to_bits();
                const EXPONENT_MASK: $bits =
                    (1 << <$float>::EXPONENT_BITS_COUNT) - 1;
                let exponent_bits = ((bits
                    >> <$float>::SIGNIFICAND_BITS_COUNT)
                    & EXPONENT_MASK) as i32;
                if exponent_bits == 0 {
                    if self == (0.0 as $float) {
                        (self, 0)
                    } else {
                        const EXPONENT_BASE: $bits = (1
                            << (<$float>::EXPONENT_BITS_COUNT - 1usize))
                            - 1;
                        const EXPONENT_DECREMENT: i32 = 64i32;
                        const SCALE: $float = unsafe {
                            transmute::<$bits, $float>(
                                (((EXPONENT_BASE as i32) + EXPONENT_DECREMENT)
                                    as $bits)
                                    << <$float>::SIGNIFICAND_BITS_COUNT,
                            )
                        };
                        let (fraction, exponent) = (self * SCALE).fract_exp();
                        (fraction, exponent - EXPONENT_DECREMENT)
                    }
                } else if exponent_bits == (EXPONENT_MASK as i32) {
                    (self, 0)
                } else {
                    const SIGNIFICANT_MASK: $bits =
                        (1 << <$float>::SIGNIFICAND_BITS_COUNT) - 1;
                    const EXPONENT_NULL_MASK: $bits = (1
                        << (<$float>::TOTAL_BITS_COUNT - 1usize))
                        | SIGNIFICANT_MASK;
                    const EXPONENT_DECREMENT: i32 = <$float>::MAX_EXP - 2i32;
                    (
                        Self::from_bits(
                            bits & EXPONENT_NULL_MASK
                                | ((EXPONENT_DECREMENT as $bits)
                                    << <$float>::SIGNIFICAND_BITS_COUNT),
                        ),
                        exponent_bits - EXPONENT_DECREMENT,
                    )
                }
            }
        }
    };
}

float_fract_exp_impl!(f32, u32);
float_fract_exp_impl!(f64, u64);
