use core::slice::{Iter, IterMut};

use super::traits::{
    Emptiable, Iterable, Lengthsome, MutIterable, ValueContainer,
};

impl<'a, Element: 'a + PartialEq> ValueContainer<'a> for [Element] {
    type Value = &'a Element;

    fn contains_value(&'a self, value: Self::Value) -> bool {
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
