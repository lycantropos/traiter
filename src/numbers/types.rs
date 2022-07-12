#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endianness {
    Big,
    Little,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TieBreaking {
    AwayFromZero,
    ToEven,
    ToOdd,
    TowardZero,
}
