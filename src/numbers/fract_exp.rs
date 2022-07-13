use core::mem::{size_of, transmute};

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

macro_rules! primitive_fract_exp_impl {
    ($f:ty, $t:ty) => {
        impl FractExp for $f {
            type Output = (Self, i32);

            #[inline]
            fn fract_exp(self) -> Self::Output {
                let bits = self.to_bits();
                const EXPONENT_MASK: $t = (1 << <$f>::EXPONENT_BITS_COUNT) - 1;
                let exponent_bits = ((bits >> <$f>::SIGNIFICAND_BITS_COUNT)
                    & EXPONENT_MASK) as i32;
                if exponent_bits == 0 {
                    if self == (0.0 as $f) {
                        (self, 0)
                    } else {
                        const EXPONENT_BASE: $t =
                            (1 << (<$f>::EXPONENT_BITS_COUNT - 1usize)) - 1;
                        const EXPONENT_DECREMENT: i32 = 64i32;
                        const SCALE: $f = unsafe {
                            transmute::<$t, $f>(
                                (((EXPONENT_BASE as i32) + EXPONENT_DECREMENT)
                                    as $t)
                                    << <$f>::SIGNIFICAND_BITS_COUNT,
                            )
                        };
                        let (fraction, exponent) = (self * SCALE).fract_exp();
                        (fraction, exponent - EXPONENT_DECREMENT)
                    }
                } else if exponent_bits == (EXPONENT_MASK as i32) {
                    (self, 0)
                } else {
                    const SIGNIFICANT_MASK: $t =
                        (1 << <$f>::SIGNIFICAND_BITS_COUNT) - 1;
                    const EXPONENT_NULL_MASK: $t = (1
                        << (<$f>::TOTAL_BITS_COUNT - 1usize))
                        | SIGNIFICANT_MASK;
                    const EXPONENT_DECREMENT: i32 = <$f>::MAX_EXP - 2i32;
                    (
                        Self::from_bits(
                            bits & EXPONENT_NULL_MASK
                                | ((EXPONENT_DECREMENT as $t)
                                    << <$f>::SIGNIFICAND_BITS_COUNT),
                        ),
                        exponent_bits - EXPONENT_DECREMENT,
                    )
                }
            }
        }
    };
}

primitive_fract_exp_impl!(f32, u32);
primitive_fract_exp_impl!(f64, u64);
