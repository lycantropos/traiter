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

impl LoadExp<i32> for f32 {
    type Output = Self;

    fn load_exp(mut self, mut exponent: i32) -> f32 {
        if exponent > 127 {
            const _0X1P127: f32 = 1.7014118e38_f32;
            self *= _0X1P127;
            exponent -= 127;
            if exponent > 127 {
                self *= _0X1P127;
                exponent -= 127;
                if exponent > 127 {
                    exponent = 127;
                }
            }
        } else if exponent < -126 {
            const _0X1P_126: f32 = 1.1754944e-38_f32;
            const _0X1P24: f32 = 16777216.0_f32;
            const SCALE: f32 = _0X1P_126 * _0X1P24;
            self *= SCALE;
            exponent += 126 - 24;
            if exponent < -126 {
                self *= SCALE;
                exponent += 126 - 24;
                if exponent < -126 {
                    exponent = -126;
                }
            }
        }
        self * f32::from_bits(((0x7f + exponent) as u32) << 23)
    }
}

impl LoadExp<i32> for f64 {
    type Output = Self;

    fn load_exp(mut self, mut exponent: i32) -> Self::Output {
        if exponent > 1023 {
            const _0X1P1023: f64 = 8.98846567431158e307_f64;
            self *= _0X1P1023;
            exponent -= 1023;
            if exponent > 1023 {
                self *= _0X1P1023;
                exponent -= 1023;
                if exponent > 1023 {
                    exponent = 1023;
                }
            }
        } else if exponent < -1022 {
            const _0X1P53: f64 = 9007199254740992.0_f64;
            const _0X1P_1022: f64 = 2.2250738585072014e-308_f64;
            const SCALE: f64 = _0X1P_1022 * _0X1P53;
            self *= SCALE;
            exponent += 1022 - 53;
            if exponent < -1022 {
                self *= SCALE;
                exponent += 1022 - 53;
                if exponent < -1022 {
                    exponent = -1022;
                }
            }
        }
        self * f64::from_bits(((0x3ff + exponent) as u64) << 52)
    }
}
