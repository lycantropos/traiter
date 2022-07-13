pub trait Zeroable {
    /// Constructs zero.
    /// ```
    /// use traiter::numbers::Zeroable;
    /// /// floating point numbers
    /// assert_eq!(<f32 as Zeroable>::zero(), 0.0_f32);
    /// /// signed integers
    /// assert_eq!(<i8 as Zeroable>::zero(), 0i8);
    /// /// unsigned integers
    /// assert_eq!(<u8 as Zeroable>::zero(), 0u8);
    /// ```
    fn zero() -> Self;

    /// Checks if a number is zero.
    /// ```
    /// use traiter::numbers::Zeroable;
    /// /// floating point numbers
    /// assert!(!Zeroable::is_zero(&-1.0_f32));
    /// assert!(Zeroable::is_zero(&0.0_f32));
    /// assert!(!Zeroable::is_zero(&1.0_f32));
    /// /// signed integers
    /// assert!(!Zeroable::is_zero(&-1i8));
    /// assert!(Zeroable::is_zero(&0i8));
    /// assert!(!Zeroable::is_zero(&1i8));
    /// /// unsigned integers
    /// assert!(Zeroable::is_zero(&0u8));
    /// assert!(!Zeroable::is_zero(&1u8));
    /// assert!(!Zeroable::is_zero(&2u8));
    /// ```
    fn is_zero(&self) -> bool;
}

macro_rules! primitive_zeroable_impl {
    ($($number:ty)*) => ($(
        impl Zeroable for $number {
            #[inline(always)]
            fn zero() -> $number {0 as $number}

            #[inline(always)]
            fn is_zero(&self) -> bool {
                *self == (0 as $number)
            }
        }
    )*)
}

primitive_zeroable_impl!(
    f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
