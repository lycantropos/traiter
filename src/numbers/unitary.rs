pub trait Unitary {
    /// Checks if a number is zero.
    /// ```
    /// use traiter::numbers::Unitary;
    /// /// floating point numbers
    /// assert!(!Unitary::is_one(-1.0_f32));
    /// assert!(!Unitary::is_one(0.0_f32));
    /// assert!(Unitary::is_one(1.0_f32));
    /// /// signed integers
    /// assert!(!Unitary::is_one(-1i8));
    /// assert!(!Unitary::is_one(0i8));
    /// assert!(Unitary::is_one(1i8));
    /// /// unsigned integers
    /// assert!(!Unitary::is_one(0u8));
    /// assert!(Unitary::is_one(1u8));
    /// assert!(!Unitary::is_one(2u8));
    /// ```
    fn is_one(self) -> bool;
}

macro_rules! number_unitary_impl {
    ($($number:ty)*) => ($(
        impl Unitary for $number {
            #[inline(always)]
            fn is_one(self) -> bool {
                self == (1 as $number)
            }
        }

        impl Unitary for &$number {
            #[inline(always)]
            fn is_one(self) -> bool {
                *self == (1 as $number)
            }
        }
    )*)
}

number_unitary_impl!(
    f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
