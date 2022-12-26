use std::collections::TryReserveError;
use std::slice::Iter;
use std::vec::Vec;

use super::traits::{
    Capacitary, Clearable, Container, Emptiable, Iterable, Lengthsome,
    Reservable, TryReservable,
};

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

impl<'a, Element: 'a> Iterable<'a> for Vec<Element> {
    type Output = Iter<'a, Element>;

    fn iter(&'a self) -> Self::Output {
        self.as_slice().iter()
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

impl<Element> TryReservable for Vec<Element> {
    type Error = TryReserveError;

    fn try_reserve(
        &mut self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        Vec::try_reserve(self, additional)
    }
}
