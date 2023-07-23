use std::collections::TryReserveError;
use std::slice::{Iter, IterMut};
use std::vec::Vec;

use super::traits::{
    Capacitary, Clearable, Emptiable, ItemInsertable, ItemRemovable, Iterable,
    Lengthsome, MutablyIterable, Reservable, TryReservable, ValueContainer,
};

impl<Element> Capacitary for Vec<Element> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        Vec::capacity(&self)
    }
}

impl<Element> Capacitary for &Vec<Element> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        Vec::capacity(self)
    }
}

impl<Element> Capacitary for &mut Vec<Element> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        Vec::capacity(self)
    }
}

impl<Element: PartialEq> Clearable for &mut Vec<Element> {
    fn clear(self) {
        Vec::clear(self)
    }
}

impl<Element> Emptiable for Vec<Element> {
    fn is_empty(self) -> bool {
        Vec::is_empty(&self)
    }
}

impl<Element> Emptiable for &Vec<Element> {
    fn is_empty(self) -> bool {
        Vec::is_empty(self)
    }
}

impl<Element> Emptiable for &mut Vec<Element> {
    fn is_empty(self) -> bool {
        Vec::is_empty(self)
    }
}

impl<Element> ItemInsertable for &mut Vec<Element> {
    type Key = usize;
    type Output = ();
    type Value = Element;

    fn insert_item(self, key: Self::Key, value: Self::Value) -> Self::Output {
        Vec::insert(self, key, value)
    }
}

impl<Element> ItemRemovable for &mut Vec<Element> {
    type Key = usize;
    type Output = Element;

    fn remove_item(self, key: Self::Key) -> Self::Output {
        Vec::remove(self, key)
    }
}

impl<'a, Element> Iterable for &'a Vec<Element> {
    type Item = &'a Element;
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        self.as_slice().iter()
    }
}

impl<'a, Element> Iterable for &'a mut Vec<Element> {
    type Item = &'a Element;
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        self.as_slice().iter()
    }
}

impl<Element> Lengthsome for Vec<Element> {
    type Length = usize;

    fn len(self) -> Self::Length {
        Vec::len(&self)
    }
}

impl<Element> Lengthsome for &Vec<Element> {
    type Length = usize;

    fn len(self) -> Self::Length {
        Vec::len(self)
    }
}

impl<Element> Lengthsome for &mut Vec<Element> {
    type Length = usize;

    fn len(self) -> Self::Length {
        Vec::len(self)
    }
}

impl<'a, Element> MutablyIterable for &'a mut Vec<Element> {
    type Item = &'a mut Element;
    type Output = IterMut<'a, Element>;

    fn iter_mut(self) -> Self::Output {
        self.as_mut_slice().iter_mut()
    }
}

impl<Element> Reservable for &mut Vec<Element> {
    fn reserve(self, additional: Self::Capacity) {
        Vec::reserve(self, additional)
    }
}

impl<Element> TryReservable for &mut Vec<Element> {
    type Error = TryReserveError;

    fn try_reserve(
        self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        Vec::try_reserve(self, additional)
    }
}

impl<'a, Element: PartialEq> ValueContainer for &'a Vec<Element> {
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        self.as_slice().contains(value)
    }
}

impl<'a, Element: PartialEq> ValueContainer for &'a mut Vec<Element> {
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        self.as_slice().contains(value)
    }
}
