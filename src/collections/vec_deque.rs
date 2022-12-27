use std::collections::vec_deque::{Iter, IterMut};
use std::collections::{TryReserveError, VecDeque};

use super::traits::{
    Capacitary, Clearable, Emptiable, ItemInsertable, ItemRemovable, Iterable,
    Lengthsome, MutIterable, Reservable, TryReservable, ValueContainer,
};

impl<Element> Capacitary for VecDeque<Element> {
    type Capacity = usize;

    fn capacity(&self) -> Self::Capacity {
        VecDeque::capacity(self)
    }
}

impl<Element: PartialEq> Clearable for VecDeque<Element> {
    fn clear(&mut self) {
        VecDeque::clear(self)
    }
}

impl<Element> Emptiable for VecDeque<Element> {
    fn is_empty(&self) -> bool {
        VecDeque::is_empty(self)
    }
}

impl<Element> ItemInsertable for VecDeque<Element> {
    type Key = usize;
    type Output = ();
    type Value = Element;

    fn insert_item(
        &mut self,
        key: Self::Key,
        value: Self::Value,
    ) -> Self::Output {
        VecDeque::insert(self, key, value)
    }
}

impl<Element> ItemRemovable<'_> for VecDeque<Element> {
    type Output = Option<Element>;
    type Key = usize;

    fn remove_item(&mut self, key: Self::Key) -> Self::Output {
        VecDeque::remove(self, key)
    }
}

impl<'a, Element: 'a> Iterable<'a> for VecDeque<Element> {
    type Output = Iter<'a, Element>;

    fn iter(&'a self) -> Self::Output {
        self.iter()
    }
}

impl<Element> Lengthsome for VecDeque<Element> {
    type Length = usize;

    fn len(&self) -> Self::Length {
        VecDeque::len(self)
    }
}

impl<'a, Element: 'a> MutIterable<'a> for VecDeque<Element> {
    type Output = IterMut<'a, Element>;

    fn iter_mut(&'a mut self) -> Self::Output {
        self.iter_mut()
    }
}

impl<Element> Reservable for VecDeque<Element> {
    fn reserve(&mut self, additional: Self::Capacity) {
        VecDeque::reserve(self, additional)
    }
}

impl<Element> TryReservable for VecDeque<Element> {
    type Error = TryReserveError;

    fn try_reserve(
        &mut self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        VecDeque::try_reserve(self, additional)
    }
}

impl<'a, Element: 'a + PartialEq> ValueContainer<'a> for VecDeque<Element> {
    type Value = &'a Element;

    fn contains_value(&'a self, value: Self::Value) -> bool {
        VecDeque::contains(self, value)
    }
}
