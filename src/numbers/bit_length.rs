pub trait BitLength {
    type Output;

    /// Returns bit length of a number.
    ///
    /// ```
    /// use traiter::numbers::BitLength;
    /// // signed integers
    /// assert_eq!(BitLength::bit_length(-1i8), 1usize);
    /// assert_eq!(BitLength::bit_length(0i8), 0usize);
    /// assert_eq!(BitLength::bit_length(1i8), 1usize);
    /// // unsigned integers
    /// assert_eq!(BitLength::bit_length(0u8), 0usize);
    /// assert_eq!(BitLength::bit_length(1u8), 1usize);
    /// assert_eq!(BitLength::bit_length(2u8), 2usize);
    /// ```
    fn bit_length(self) -> Self::Output;
}

macro_rules! primitive_signed_bit_length_impl {
    ($($t:ty)*) => ($(
        impl BitLength for $t {
            type Output = usize;

            #[inline(always)]
            fn bit_length(self) -> Self::Output {
                (<$t>::BITS as usize) - (self.abs().leading_zeros() as usize)
            }
        }
    )*)
}

primitive_signed_bit_length_impl!(i8 i16 i32 i64 i128 isize);

macro_rules! primitive_unsigned_bit_length_impl {
    ($($t:ty)*) => ($(
        impl BitLength for $t {
            type Output = usize;

            #[inline(always)]
            fn bit_length(self) -> Self::Output {
                (<$t>::BITS as usize) - (self.leading_zeros() as usize)
            }
        }
    )*)
}

primitive_unsigned_bit_length_impl!(u8 u16 u32 u64 u128 usize);
