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

impl FractExp for f32 {
    type Output = (Self, i32);

    fn fract_exp(self) -> Self::Output {
        let bits = self.to_bits();
        let exponent_bits = ((bits >> 23) & 0xff) as i32;
        if exponent_bits == 0 {
            if self == 0.0_f32 {
                (self, 0)
            } else {
                const _0X1P64: f32 = 1.8446744e19_f32;
                let (fraction, exponent) = (self * _0X1P64).fract_exp();
                (fraction, exponent - 64)
            }
        } else if exponent_bits == 0xff {
            (self, 0)
        } else {
            (
                f32::from_bits(bits & 0x807fffff | 0x3f000000),
                exponent_bits - 0x7e,
            )
        }
    }
}

impl FractExp for f64 {
    type Output = (Self, i32);

    fn fract_exp(self) -> Self::Output {
        let bits = self.to_bits();
        let exponent_bits = ((bits >> 52) & 0x7ff) as i32;
        if exponent_bits == 0 {
            if self == 0.0_f64 {
                (self, 0)
            } else {
                const _0X1P64: f64 = 1.8446744073709552e19_f64;
                let (fraction, exponent) = (self * _0X1P64).fract_exp();
                (fraction, exponent - 64)
            }
        } else if exponent_bits == 0x7ff {
            (self, 0)
        } else {
            (
                f64::from_bits(bits & 0x800fffffffffffff | 0x3fe0000000000000),
                exponent_bits - 0x3fe,
            )
        }
    }
}
