use std::ops::Rem;

pub trait MulRem<Other = Self, Divisor = Self> {
    type Output;

    /// Returns remainder of multiplication followed by division.
    /// ```
    /// use traiter::numbers::MulRem;
    /// // signed integers
    /// assert_eq!(MulRem::mul_rem(-3i8, 1i8, 2i8), -1i8);
    /// assert_eq!(MulRem::mul_rem(-3i8, 1i8, 1i8), 0i8);
    /// assert_eq!(MulRem::mul_rem(i8::MIN, -1i8, i8::MAX), 1i8);
    /// // unsigned integers
    /// assert_eq!(MulRem::mul_rem(2u8, 1u8, 3u8), 2u8);
    /// assert_eq!(MulRem::mul_rem(2u8, 1u8, 2u8), 0u8);
    /// assert_eq!(MulRem::mul_rem(2u8, 1u8, 1u8), 0u8);
    /// ```
    fn mul_rem(self, other: Other, divisor: Divisor) -> Self::Output;
}

macro_rules! unsigned_integer_mul_rem_impl {
    ($($integer:ty)*) => ($(
        impl MulRem for $integer {
            type Output = Self;

            fn mul_rem(self, other: Self, divisor: Self) -> Self::Output {
                if divisor <= 1 << (Self::BITS / 2) {
                    ((self.rem(divisor)) * (other.rem(divisor))).rem(divisor)
                } else {
                    let add = |left: Self, right: Self| -> Self {
                        left.checked_sub(divisor - right)
                            .unwrap_or_else(|| left + right)
                    };
                    let split = |value: Self| -> (Self, Self) {
                        (
                            value >> (Self::BITS / 2),
                            value & ((1 << (Self::BITS / 2)) - 1),
                        )
                    };
                    let (high, low) = split(self);
                    let (other_high, other_low) = split(other);
                    let mut result = (high * other_high).rem(divisor);
                    let (low_times_other_high_high, low_times_other_high_low) =
                        split(low * other_high);
                    result = add(result, low_times_other_high_high);
                    let (high_times_other_low_high, high_times_other_low_low) =
                        split(high * other_low);
                    result = add(result, high_times_other_low_high);
                    for _ in 0..(Self::BITS / 2) {
                        result = add(result, result);
                    }
                    result = add(result, low_times_other_high_low);
                    result = add(result, high_times_other_low_low);
                    let (low_times_other_low_high, low_times_other_low_low) =
                        split(low * other_low);
                    result = add(result, low_times_other_low_high);
                    for _ in 0..(Self::BITS / 2) {
                        result = add(result, result);
                    }
                    add(result, low_times_other_low_low)
                }
            }
        }
    )*)
}

unsigned_integer_mul_rem_impl!(u8 u16 u32 u64 u128 usize);

trait Unsigned {
    type Result;
}

impl Unsigned for i8 {
    type Result = u8;
}

impl Unsigned for i16 {
    type Result = u16;
}

impl Unsigned for i32 {
    type Result = u32;
}

impl Unsigned for i64 {
    type Result = u64;
}

impl Unsigned for i128 {
    type Result = u128;
}

impl Unsigned for isize {
    type Result = usize;
}

trait UnsignedAbs {
    type Output;

    fn unsigned_abs(self) -> Self::Output;
}

macro_rules! signed_integer_unsigned_abs_impl {
    ($($integer:ty)*) => ($(
        impl UnsignedAbs for $integer {
            type Output = <$integer as Unsigned>::Result;

            fn unsigned_abs(self) -> Self::Output {
                self.checked_abs()
                    .map(|value| value as Self::Output)
                    .unwrap_or(
                        (Self::MAX as Self::Output) + 1,
                    )
            }
        }
    )*)
}

signed_integer_unsigned_abs_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! signed_integer_mul_rem_impl {
    ($($integer:ty)*) => ($(
        impl MulRem for $integer {
            type Output = Self;

            fn mul_rem(self, other: Self, divisor: Self) -> Self::Output {
                let result = (self
                    .unsigned_abs()
                    .mul_rem(other.unsigned_abs(), divisor.unsigned_abs())
                    as Self);
                if self.is_negative() != other.is_negative() {
                    -result
                } else {
                    result
                }
            }
        }
    )*)
}

signed_integer_mul_rem_impl!(i8 i16 i32 i64 i128 isize);
