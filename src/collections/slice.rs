use core::slice::{Iter, IterMut};

use super::traits::{Container, Emptiable, Iterable, Lengthsome, MutIterable};

impl<Element: PartialEq> Container for [Element] {
    type Value = Element;

    fn contains(&self, value: &Self::Value) -> bool {
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

impl<'a, Element: 'a> MutIterable<'a> for [Element] {
    type Output = IterMut<'a, Element>;

    fn iter_mut(&'a mut self) -> Self::Output {
        <[Element]>::iter_mut(self)
    }
}
