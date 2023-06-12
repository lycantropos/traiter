pub trait One {
    /// Constructs one.
    /// ```
    /// use traiter::numbers::One;
    /// /// floating point numbers
    /// assert_eq!(<f32 as One>::one(), 1.0_f32);
    /// /// signed integers
    /// assert_eq!(<i8 as One>::one(), 1i8);
    /// /// unsigned integers
    /// assert_eq!(<u8 as One>::one(), 1u8);
    /// ```
    fn one() -> Self;
}

macro_rules! number_one_impl {
    ($($number:ty)*) => ($(
        impl One for $number {
            #[inline(always)]
            fn one() -> $number {1 as $number}
        }
    )*)
}

number_one_impl!(
    f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
