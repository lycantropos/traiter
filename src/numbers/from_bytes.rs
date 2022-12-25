use core::convert::TryInto;

use super::types::Endianness;

pub trait FromBytes {
    /// Creates number from its byte array representation in given endianness.
    /// ```
    /// use traiter::numbers::{Endianness, FromBytes};
    /// /// signed integers
    /// assert_eq!(
    ///     <i8 as FromBytes>::from_bytes(&[255u8], Endianness::Big), -1i8
    /// );
    /// assert_eq!(
    ///     <i8 as FromBytes>::from_bytes(&[0u8], Endianness::Big), 0i8
    /// );
    /// assert_eq!(
    ///     <i8 as FromBytes>::from_bytes(&[1u8], Endianness::Big), 1i8
    /// );
    /// /// unsigned integers
    /// assert_eq!(
    ///     <u8 as FromBytes>::from_bytes(&[0u8], Endianness::Big), 0u8
    /// );
    /// assert_eq!(
    ///     <u8 as FromBytes>::from_bytes(&[1u8], Endianness::Big), 1u8
    /// );
    /// assert_eq!(
    ///     <u8 as FromBytes>::from_bytes(&[2u8], Endianness::Big), 2u8
    /// );
    /// ```
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self;
}

macro_rules! integer_from_bytes_impl {
    ($($integer:ty)*) => ($(
        impl FromBytes for $integer {
            #[inline(always)]
            fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
                match endianness {
                    Endianness::Big => {
                        <$integer>::from_be_bytes(bytes.try_into().unwrap())
                    }
                    Endianness::Little => {
                        <$integer>::from_le_bytes(bytes.try_into().unwrap())
                    }
                }
            }
        }
    )*)
}

integer_from_bytes_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
