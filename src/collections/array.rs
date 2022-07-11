use super::traits::{Container, Emptiable, Lengthsome};

impl<Element: PartialEq, const LENGTH: usize> Container<&Element>
    for [Element; LENGTH]
{
    fn contains(&self, value: &Element) -> bool {
        <[Element]>::contains(self, value)
    }
}

impl<Element, const LENGTH: usize> Emptiable for [Element; LENGTH] {
    fn is_empty(&self) -> bool {
        LENGTH == 0
    }
}

impl<Element, const LENGTH: usize> Lengthsome for [Element; LENGTH] {
    type Length = usize;

    fn len(&self) -> Self::Length {
        LENGTH
    }
}
