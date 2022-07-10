pub trait DivEuclid<Divisor = Self> {
    type Output;

    fn div_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_div_euclid_impl {
    ($($t:ty)*) => ($(
        impl DivEuclid for $t {
            type Output = Self;

            #[inline(always)]
            fn div_euclid(self, divisor: Self) -> Self::Output {
                <$t>::div_euclid(self, divisor)
            }
        }
    )*)
}

primitive_div_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
