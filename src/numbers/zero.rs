pub trait Zero {
    /// Constructs zero.
    /// ```
    /// use traiter::numbers::Zero;
    /// /// floating point numbers
    /// assert_eq!(<f32 as Zero>::zero(), 0.0_f32);
    /// /// signed integers
    /// assert_eq!(<i8 as Zero>::zero(), 0i8);
    /// /// unsigned integers
    /// assert_eq!(<u8 as Zero>::zero(), 0u8);
    /// ```
    fn zero() -> Self;
}

macro_rules! number_zero_impl {
    ($($number:ty)*) => ($(
        impl Zero for $number {
            #[inline(always)]
            fn zero() -> $number {0 as $number}
        }
    )*)
}

number_zero_impl!(
    f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
