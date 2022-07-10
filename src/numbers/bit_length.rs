pub trait BitLength {
    type Output;

    fn bit_length(self) -> Self::Output;
}

macro_rules! primitive_bit_length_impl {
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

primitive_bit_length_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
