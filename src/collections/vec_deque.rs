use std::collections::vec_deque::{Iter, IterMut};
use std::collections::{TryReserveError, VecDeque};

use super::traits::{
    Capacitary, Clearable, Emptiable, ItemInsertable, ItemRemovable, Iterable,
    Lengthsome, MutablyIterable, Reservable, TryReservable, ValueContainer,
};

impl<Element> Capacitary for VecDeque<Element> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        VecDeque::capacity(&self)
    }
}

impl<Element> Capacitary for &VecDeque<Element> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        VecDeque::capacity(self)
    }
}

impl<Element> Capacitary for &mut VecDeque<Element> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        VecDeque::capacity(self)
    }
}

impl<Element: PartialEq> Clearable for &mut VecDeque<Element> {
    fn clear(self) {
        VecDeque::clear(self)
    }
}

impl<Element> Emptiable for VecDeque<Element> {
    fn is_empty(self) -> bool {
        VecDeque::is_empty(&self)
    }
}

impl<Element> Emptiable for &VecDeque<Element> {
    fn is_empty(self) -> bool {
        VecDeque::is_empty(self)
    }
}

impl<Element> Emptiable for &mut VecDeque<Element> {
    fn is_empty(self) -> bool {
        VecDeque::is_empty(self)
    }
}

impl<Element> ItemInsertable for &mut VecDeque<Element> {
    type Key = usize;
    type Output = ();
    type Value = Element;

    fn insert_item(self, key: Self::Key, value: Self::Value) -> Self::Output {
        VecDeque::insert(self, key, value)
    }
}

impl<Element> ItemRemovable for &mut VecDeque<Element> {
    type Output = Option<Element>;
    type Key = usize;

    fn remove_item(self, key: Self::Key) -> Self::Output {
        VecDeque::remove(self, key)
    }
}

impl<'a, Element> Iterable for &'a VecDeque<Element> {
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        VecDeque::iter(self)
    }
}

impl<'a, Element> Iterable for &'a mut VecDeque<Element> {
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        VecDeque::iter(self)
    }
}

impl<Element> Lengthsome for VecDeque<Element> {
    type Length = usize;

    fn len(self) -> Self::Length {
        VecDeque::len(&self)
    }
}

impl<Element> Lengthsome for &VecDeque<Element> {
    type Length = usize;

    fn len(self) -> Self::Length {
        VecDeque::len(self)
    }
}

impl<Element> Lengthsome for &mut VecDeque<Element> {
    type Length = usize;

    fn len(self) -> Self::Length {
        VecDeque::len(self)
    }
}

impl<'a, Element> MutablyIterable for &'a mut VecDeque<Element> {
    type Output = IterMut<'a, Element>;

    fn iter_mut(self) -> Self::Output {
        VecDeque::iter_mut(self)
    }
}

impl<Element> Reservable for &mut VecDeque<Element> {
    fn reserve(self, additional: Self::Capacity) {
        VecDeque::reserve(self, additional)
    }
}

impl<Element> TryReservable for &mut VecDeque<Element> {
    type Error = TryReserveError;

    fn try_reserve(
        self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        VecDeque::try_reserve(self, additional)
    }
}

impl<'a, Element: PartialEq> ValueContainer for &'a VecDeque<Element> {
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        VecDeque::contains(self, value)
    }
}

impl<'a, Element: PartialEq> ValueContainer for &'a mut VecDeque<Element> {
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        VecDeque::contains(self, value)
    }
}
