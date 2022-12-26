use core::slice::{Iter, IterMut};

use super::traits::{Container, Emptiable, Iterable, Lengthsome, MutIterable};

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

impl<'a, Element: 'a, const LENGTH: usize> Iterable<'a> for [Element; LENGTH] {
    type Output = Iter<'a, Element>;

    fn iter(&'a self) -> Self::Output {
        self.as_slice().iter()
    }
}

impl<Element, const LENGTH: usize> Lengthsome for [Element; LENGTH] {
    type Length = usize;

    fn len(&self) -> Self::Length {
        LENGTH
    }
}

impl<'a, Element: 'a, const LENGTH: usize> MutIterable<'a>
    for [Element; LENGTH]
{
    type Output = IterMut<'a, Element>;

    fn iter_mut(&'a mut self) -> Self::Output {
        self.as_mut_slice().iter_mut()
    }
}
