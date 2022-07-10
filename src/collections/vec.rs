use super::traits::{Capacitary, Clearable, Container, Emptiable, Lengthsome, Reservable};

impl<Element> Capacitary for Vec<Element> {
    type Capacity = usize;

    fn capacity(&self) -> Self::Capacity {
        Vec::capacity(self)
    }
}

impl<Element: PartialEq> Clearable for Vec<Element> {
    fn clear(&mut self) {
        Vec::clear(self)
    }
}

impl<Element: PartialEq> Container<&Element> for Vec<Element> {
    fn contains(&self, value: &Element) -> bool {
        self.as_slice().contains(value)
    }
}

impl<Element> Emptiable for Vec<Element> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<Element> Lengthsome for Vec<Element> {
    type Length = usize;

    fn len(&self) -> Self::Length {
        Vec::len(self)
    }
}

impl<Element> Reservable for Vec<Element> {
    fn reserve(&mut self, additional: Self::Capacity) {
        Vec::reserve(self, additional)
    }
}
