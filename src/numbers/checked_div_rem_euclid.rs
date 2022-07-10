pub trait CheckedDivRemEuclid<Divisor = Self> {
    type Output;

    fn checked_div_rem_euclid(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_checked_div_rem_euclid_impl {
    ($($t:ty)*) => ($(
        impl CheckedDivRemEuclid for $t {
            type Output = Option<(Self, Self)>;

            #[inline(always)]
            fn checked_div_rem_euclid(self, divisor: Self) -> Self::Output {
                if divisor == 0 {
                    None
                } else {
                    Some((<$t>::div_euclid(self, divisor), <$t>::rem_euclid(self, divisor)))
                }
            }
        }
    )*)
}

primitive_checked_div_rem_euclid_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
