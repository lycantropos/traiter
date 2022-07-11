pub trait BitLength {
    type Output;

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
