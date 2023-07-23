use core::slice::{Iter, IterMut};

use super::traits::{
    Emptiable, Iterable, Lengthsome, MutablyIterable, ValueContainer,
};

impl<Element> Emptiable for &[Element] {
    fn is_empty(self) -> bool {
        <[Element]>::is_empty(self)
    }
}

impl<Element> Emptiable for &mut [Element] {
    fn is_empty(self) -> bool {
        <[Element]>::is_empty(self)
    }
}

impl<'a, Element> Iterable for &'a [Element] {
    type Item = &'a Element;
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        <[Element]>::iter(self)
    }
}

impl<'a, Element> Iterable for &'a mut [Element] {
    type Item = &'a Element;
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        <[Element]>::iter(self)
    }
}

impl<Element> Lengthsome for &[Element] {
    type Length = usize;

    fn len(self) -> Self::Length {
        <[Element]>::len(self)
    }
}

impl<Element> Lengthsome for &mut [Element] {
    type Length = usize;

    fn len(self) -> Self::Length {
        <[Element]>::len(self)
    }
}

impl<'a, Element> MutablyIterable for &'a mut [Element] {
    type Item = &'a mut Element;
    type Output = IterMut<'a, Element>;

    fn iter_mut(self) -> Self::Output {
        <[Element]>::iter_mut(self)
    }
}

impl<'a, Element: PartialEq> ValueContainer for &'a [Element] {
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        <[Element]>::contains(self, value)
    }
}

impl<'a, Element: PartialEq> ValueContainer for &'a mut [Element] {
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        <[Element]>::contains(self, value)
    }
}
