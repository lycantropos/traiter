use core::convert::TryInto;
use core::fmt::{Debug, Display, Formatter};

use super::types::Endianness;

pub trait TryFromBytes: Sized {
    /// Creates number from its byte array representation in given endianness.
    /// ```
    /// use traiter::numbers::{Endianness, TryFromBytes};
    /// /// signed integers
    /// assert_eq!(
    ///     <i8 as TryFromBytes>::try_from_bytes(&[255u8], Endianness::Big),
    ///     Ok(-1i8)
    /// );
    /// assert_eq!(
    ///     <i8 as TryFromBytes>::try_from_bytes(&[0u8], Endianness::Big),
    ///     Ok(0i8)
    /// );
    /// assert_eq!(
    ///     <i8 as TryFromBytes>::try_from_bytes(&[1u8], Endianness::Big),
    ///     Ok(1i8)
    /// );
    /// assert!(
    ///     <i8 as TryFromBytes>::try_from_bytes(&[], Endianness::Big).is_err()
    /// );
    /// /// unsigned integers
    /// assert_eq!(
    ///     <u8 as TryFromBytes>::try_from_bytes(&[0u8], Endianness::Big),
    ///     Ok(0u8)
    /// );
    /// assert_eq!(
    ///     <u8 as TryFromBytes>::try_from_bytes(&[1u8], Endianness::Big),
    ///     Ok(1u8)
    /// );
    /// assert_eq!(
    ///     <u8 as TryFromBytes>::try_from_bytes(&[2u8], Endianness::Big),
    ///     Ok(2u8)
    /// );
    /// assert!(
    ///     <u8 as TryFromBytes>::try_from_bytes(&[], Endianness::Big).is_err()
    /// );
    /// ```

    type Error;

    fn try_from_bytes(
        bytes: &[u8],
        endianness: Endianness,
    ) -> Result<Self, Self::Error>;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TryFromBytesError {
    NotEnoughBytes,
    TooManyBytes,
}

impl TryFromBytesError {
    fn description(&self) -> &str {
        match self {
            TryFromBytesError::NotEnoughBytes => {
                "Not enough bytes were provided."
            }
            TryFromBytesError::TooManyBytes => "Too many bytes were provided.",
        }
    }
}

impl Debug for TryFromBytesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.description())
    }
}

impl Display for TryFromBytesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.description(), formatter)
    }
}

macro_rules! integer_try_from_bytes_impl {
    ($($integer:ty)*) => ($(
        impl TryFromBytes for $integer {
            type Error = TryFromBytesError;

            #[inline(always)]
            fn try_from_bytes(
                bytes: &[u8],
                endianness: Endianness,
            ) -> Result<Self, Self::Error> {
                bytes
                    .try_into()
                    .map(|value| match endianness {
                        Endianness::Big => <$integer>::from_be_bytes(value),
                        Endianness::Little => <$integer>::from_le_bytes(value),
                    })
                    .map_err(|_| {
                        if bytes.len() < (<$integer>::BITS as usize) / 8 {
                            TryFromBytesError::NotEnoughBytes
                        } else {
                            TryFromBytesError::TooManyBytes
                        }
                    })
            }
        }
    )*)
}

integer_try_from_bytes_impl!(
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
);
