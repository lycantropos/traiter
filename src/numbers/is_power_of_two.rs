pub trait IsPowerOfTwo {
    /// Checks if a number is a power of 2.
    /// ```
    /// use traiter::numbers::IsPowerOfTwo;
    /// // signed integers
    /// assert!(!IsPowerOfTwo::is_power_of_two(&-1i8));
    /// assert!(!IsPowerOfTwo::is_power_of_two(&0i8));
    /// assert!(IsPowerOfTwo::is_power_of_two(&1i8));
    /// // unsigned integers
    /// assert!(!IsPowerOfTwo::is_power_of_two(&0u8));
    /// assert!(IsPowerOfTwo::is_power_of_two(&1u8));
    /// assert!(IsPowerOfTwo::is_power_of_two(&2i8));
    /// ```
    fn is_power_of_two(&self) -> bool;
}

macro_rules! unsigned_integer_is_power_of_two_impl {
    ($($integer:ty)*) => ($(
        impl IsPowerOfTwo for $integer {
            #[inline(always)]
            fn is_power_of_two(&self) -> bool {
                <$integer>::is_power_of_two(*self)
            }
        }
    )*)
}

unsigned_integer_is_power_of_two_impl!(u8 u16 u32 u64 u128 usize);

trait Unsigned {
    type Output;
}

impl Unsigned for i8 {
    type Output = u8;
}

impl Unsigned for i16 {
    type Output = u8;
}

impl Unsigned for i32 {
    type Output = u16;
}

impl Unsigned for i64 {
    type Output = u32;
}

impl Unsigned for i128 {
    type Output = u64;
}

impl Unsigned for isize {
    type Output = usize;
}

macro_rules! signed_integer_is_power_of_two_impl {
    ($($integer:ty)*) => ($(
        impl IsPowerOfTwo for $integer {
            #[inline(always)]
            fn is_power_of_two(&self) -> bool {
                <$integer>::is_positive(*self)
                    && (*self as <$integer as Unsigned>::Output)
                        .is_power_of_two()
            }
        }
    )*)
}

signed_integer_is_power_of_two_impl!(i8 i16 i32 i64 i128 isize);
