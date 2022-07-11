pub trait CheckedDiv<Divisor = Self> {
    type Output;

    fn checked_div(self, divisor: Divisor) -> Self::Output;
}

macro_rules! primitive_checked_div_impl {
    ($($t:ty)*) => ($(
        impl CheckedDiv for $t {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_div(self, divisor: Self) -> Self::Output {
                <$t>::checked_div(self, divisor)
            }
        }
    )*)
}

primitive_checked_div_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
