use super::traits::{Container, Emptiable, Lengthsome};

impl<Element: PartialEq> Container<&Element> for &Vec<Element> {
    fn contains(self, value: &Element) -> bool {
        self.as_slice().contains(value)
    }
}

impl<Element> Emptiable for &Vec<Element> {
    fn is_empty(self) -> bool {
        Vec::is_empty(self)
    }
}

impl<Element> Lengthsome for &Vec<Element> {
    type Length = usize;

    fn len(self) -> Self::Length {
        Vec::len(self)
    }
}
