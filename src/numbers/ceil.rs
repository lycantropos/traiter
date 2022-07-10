pub trait Ceil {
    type Output;

    fn ceil(self) -> Self::Output;
}

macro_rules! primitive_ceil_impl {
    ($($t:ty)*) => ($(
        impl Ceil for $t {
            type Output = $t;

            #[inline(always)]
            fn ceil(self) -> Self::Output {
                <$t>::ceil(self)
            }
        }
    )*)
}

primitive_ceil_impl!(f32 f64);
