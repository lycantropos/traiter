use core::slice::{Iter, IterMut};

use super::traits::{
    Emptiable, Iterable, Lengthsome, MutablyIterable, ValueContainer,
};

impl<Element, const LENGTH: usize> Emptiable for [Element; LENGTH] {
    fn is_empty(self) -> bool {
        LENGTH == 0
    }
}

impl<Element, const LENGTH: usize> Emptiable for &[Element; LENGTH] {
    fn is_empty(self) -> bool {
        LENGTH == 0
    }
}

impl<Element, const LENGTH: usize> Emptiable for &mut [Element; LENGTH] {
    fn is_empty(self) -> bool {
        LENGTH == 0
    }
}

impl<'a, Element, const LENGTH: usize> Iterable for &'a [Element; LENGTH] {
    type Item = &'a Element;
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        self.as_slice().iter()
    }
}

impl<'a, Element, const LENGTH: usize> Iterable for &'a mut [Element; LENGTH] {
    type Item = &'a Element;
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        self.as_slice().iter()
    }
}

impl<Element, const LENGTH: usize> Lengthsome for [Element; LENGTH] {
    type Length = usize;

    fn len(self) -> Self::Length {
        LENGTH
    }
}

impl<Element, const LENGTH: usize> Lengthsome for &[Element; LENGTH] {
    type Length = usize;

    fn len(self) -> Self::Length {
        LENGTH
    }
}

impl<Element, const LENGTH: usize> Lengthsome for &mut [Element; LENGTH] {
    type Length = usize;

    fn len(self) -> Self::Length {
        LENGTH
    }
}

impl<'a, Element, const LENGTH: usize> MutablyIterable
    for &'a mut [Element; LENGTH]
{
    type Item = &'a mut Element;
    type Output = IterMut<'a, Element>;

    fn iter_mut(self) -> Self::Output {
        self.as_mut_slice().iter_mut()
    }
}

impl<'a, Element: PartialEq, const LENGTH: usize> ValueContainer
    for &'a [Element; LENGTH]
{
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        <[Element]>::contains(self, value)
    }
}

impl<'a, Element: PartialEq, const LENGTH: usize> ValueContainer
    for &'a mut [Element; LENGTH]
{
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        <[Element]>::contains(self, value)
    }
}
