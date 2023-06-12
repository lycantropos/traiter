pub trait Parity {
    /// Checks if a number is even.
    /// ```
    /// use traiter::numbers::Parity;
    /// // signed integers
    /// assert!(!Parity::is_even(-1i8));
    /// assert!(Parity::is_even(0i8));
    /// assert!(!Parity::is_even(1i8));
    /// // unsigned integers
    /// assert!(Parity::is_even(0u8));
    /// assert!(!Parity::is_even(1i8));
    /// assert!(Parity::is_even(2i8));
    /// ```
    fn is_even(self) -> bool;

    /// Checks if a number is odd.
    /// ```
    /// use traiter::numbers::Parity;
    /// // signed integers
    /// assert!(Parity::is_odd(-1i8));
    /// assert!(!Parity::is_odd(0i8));
    /// assert!(Parity::is_odd(1i8));
    /// // unsigned integers
    /// assert!(!Parity::is_odd(0u8));
    /// assert!(Parity::is_odd(1i8));
    /// assert!(!Parity::is_odd(2i8));
    /// ```
    fn is_odd(self) -> bool;
}

macro_rules! integer_parity_impl {
    ($($integer:ty)*) => ($(
        impl Parity for $integer {
            #[inline(always)]
            fn is_even(self) -> bool {
                self & 1 == 0
            }

            #[inline(always)]
            fn is_odd(self) -> bool {
                self & 1 == 1
            }
        }

        impl Parity for &$integer {
            #[inline(always)]
            fn is_even(self) -> bool {
                self & 1 == 0
            }

            #[inline(always)]
            fn is_odd(self) -> bool {
                self & 1 == 1
            }
        }
    )*)
}

integer_parity_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
