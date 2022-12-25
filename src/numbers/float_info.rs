use core::mem::size_of;

pub trait FloatInfo {
    const EXPONENT_BITS_COUNT: usize;
    const SIGN_BITS_COUNT: usize;
    const SIGNIFICAND_BITS_COUNT: usize;
    const TOTAL_BITS_COUNT: usize;
}

macro_rules! float_float_info_impl {
    ($($float:ty)*) => ($(
        impl FloatInfo for $float {
            const EXPONENT_BITS_COUNT: usize = Self::TOTAL_BITS_COUNT
                - Self::SIGNIFICAND_BITS_COUNT
                - Self::SIGN_BITS_COUNT;
            const SIGN_BITS_COUNT: usize = 1usize;
            const SIGNIFICAND_BITS_COUNT: usize =
                (Self::MANTISSA_DIGITS as usize) - Self::SIGN_BITS_COUNT;
            const TOTAL_BITS_COUNT: usize = size_of::<Self>() * 8usize;
        }
    )*)
}

float_float_info_impl!(f32 f64);
