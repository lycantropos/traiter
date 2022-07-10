use core::mem;

use super::types::Endianness;

pub trait ToBytes {
    type Output;

    fn to_bytes(&self, endianness: Endianness) -> Self::Output;
}

macro_rules! primitive_to_bytes_impl {
    ($($t:ty)*) => ($(
        impl ToBytes for $t {
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

primitive_to_bytes_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
