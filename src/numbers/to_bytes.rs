use core::mem;

use super::types::Endianness;

pub trait ToBytes {
    type Output;

    /// Returns representation of a number as a byte array in given endianness.
    /// ```
    /// use traiter::numbers::{ToBytes, Endianness};
    /// /// signed integers
    /// assert_eq!(ToBytes::to_bytes(&-1i8, Endianness::Little), [255u8]);
    /// assert_eq!(ToBytes::to_bytes(&0i8, Endianness::Little), [0u8]);
    /// assert_eq!(ToBytes::to_bytes(&1i8, Endianness::Little), [1u8]);
    /// /// unsigned integers
    /// assert_eq!(ToBytes::to_bytes(&0u8, Endianness::Little), [0u8]);
    /// assert_eq!(ToBytes::to_bytes(&1u8, Endianness::Little), [1u8]);
    /// assert_eq!(ToBytes::to_bytes(&2u8, Endianness::Little), [2u8]);
    /// ```
    fn to_bytes(&self, endianness: Endianness) -> Self::Output;
}

macro_rules! integer_to_bytes_impl {
    ($($integer:ty)*) => ($(
        impl ToBytes for $integer {
            type Output = [u8; mem::size_of::<Self>()];

            #[inline(always)]
            fn to_bytes(&self, endianness: Endianness) -> Self::Output {
                match endianness {
                   Endianness::Big => self.to_be_bytes(),
                   Endianness::Little => self.to_le_bytes(),
                }
            }
        }
    )*)
}

integer_to_bytes_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
