use core::slice::Iter;

use super::traits::{Container, Emptiable, Iterable, Lengthsome};

impl<Element: PartialEq> Container<&Element> for [Element] {
    fn contains(&self, value: &Element) -> bool {
        <[Element]>::contains(self, value)
    }
}

impl<Element> Emptiable for [Element] {
    fn is_empty(&self) -> bool {
        <[Element]>::is_empty(self)
    }
}

impl<'a, Element: 'a> Iterable<'a> for [Element] {
    type Output = Iter<'a, Element>;

    fn iter(&'a self) -> Self::Output {
        <[Element]>::iter(self)
    }
}

impl<Element> Lengthsome for [Element] {
    type Length = usize;

    fn len(&self) -> Self::Length {
        <[Element]>::len(self)
    }
}
