use core::convert::TryFrom;

pub trait CheckedShr<Shift = Self> {
    type Output;

    /// Returns checked shift-right.
    /// ```
    /// use traiter::numbers::CheckedShr;
    /// // signed integers
    /// assert_eq!(CheckedShr::checked_shr(-3i8, -1i8), None);
    /// assert_eq!(CheckedShr::checked_shr(-3i8, 0i8), Some(-3i8));
    /// assert_eq!(CheckedShr::checked_shr(-3i8, 1i8), Some(-2i8));
    /// // unsigned integers
    /// assert_eq!(CheckedShr::checked_shr(3u8, 0u8), Some(3u8));
    /// assert_eq!(CheckedShr::checked_shr(3u8, 1u8), Some(1u8));
    /// assert_eq!(CheckedShr::checked_shr(3u8, 2u8), Some(0u8));
    /// ```
    fn checked_shr(self, shift: Shift) -> Self::Output;
}

macro_rules! signed_checked_shr_impl {
    ($base:ty, $shift:ty) => {
        impl CheckedShr<$shift> for $base {
            type Output = Option<$base>;

            #[inline(always)]
            fn checked_shr(self, shift: $shift) -> Self::Output {
                if shift < 0 {
                    None
                } else {
                    <$base>::checked_shr(self, u32::try_from(shift).ok()?)
                }
            }
        }
    };
}

macro_rules! unsigned_checked_shr_impl {
    ($base:ty, $shift:ty) => {
        impl CheckedShr<$shift> for $base {
            type Output = Option<Self>;

            #[inline(always)]
            fn checked_shr(self, shift: $shift) -> Self::Output {
                <$base>::checked_shr(self, u32::try_from(shift).ok()?)
            }
        }
    };
}

macro_rules! primitive_checked_shr_impl {
    ($($base:ty)*) => ($(
        signed_checked_shr_impl! { $base, i8 }
        signed_checked_shr_impl! { $base, i16 }
        signed_checked_shr_impl! { $base, i32 }
        signed_checked_shr_impl! { $base, i64 }
        signed_checked_shr_impl! { $base, i128 }
        signed_checked_shr_impl! { $base, isize }

        unsigned_checked_shr_impl! { $base, u8 }
        unsigned_checked_shr_impl! { $base, u16 }
        unsigned_checked_shr_impl! { $base, u32 }
        unsigned_checked_shr_impl! { $base, u64 }
        unsigned_checked_shr_impl! { $base, u128 }
        unsigned_checked_shr_impl! { $base, usize }
    )*)
}

primitive_checked_shr_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
