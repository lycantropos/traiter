pub trait CheckedAdd<Other = Self> {
    type Output;

    /// Returns checked addition.
    /// ```
    /// use traiter::numbers::CheckedAdd;
    /// // signed integers
    /// assert_eq!(CheckedAdd::checked_add(i8::MIN, -1i8), None);
    /// assert_eq!(CheckedAdd::checked_add(0i8, 1i8), Some(1i8));
    /// assert_eq!(CheckedAdd::checked_add(i8::MAX, 1i8), None);
    /// // unsigned integers
    /// assert_eq!(CheckedAdd::checked_add(0u8, 1u8), Some(1u8));
    /// assert_eq!(CheckedAdd::checked_add(1u8, 1u8), Some(2u8));
    /// assert_eq!(CheckedAdd::checked_add(u8::MAX, 1u8), None);
    /// ```
    fn checked_add(self, other: Other) -> Self::Output;
}

macro_rules! integer_checked_add_impl {
    ($($integer:ty)*) => ($(
        impl CheckedAdd for $integer {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_add(self, other: Self) -> Self::Output {
                <$integer>::checked_add(self, other)
            }
        }
    )*)
}

integer_checked_add_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
