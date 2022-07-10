use core::convert::TryInto;

use super::types::Endianness;

pub trait FromBytes {
    fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self;
}

macro_rules! primitive_from_bytes_impl {
    ($($t:ty)*) => ($(
        impl FromBytes for $t {
            #[inline(always)]
            fn from_bytes(bytes: &[u8], endianness: Endianness) -> Self {
                match endianness {
                   Endianness::Big => <$t>::from_be_bytes(bytes.try_into().unwrap()),
                   Endianness::Little => <$t>::from_le_bytes(bytes.try_into().unwrap()),
                }
            }
        }
    )*)
}

primitive_from_bytes_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
