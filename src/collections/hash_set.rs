use std::collections::hash_set::Iter;
use std::collections::{HashSet, TryReserveError};
use std::hash::{BuildHasher, Hash};

use super::traits::{
    Capacitary, Clearable, Emptiable, Iterable, Lengthsome, Reservable,
    TryReservable, ValueContainer, ValueInsertable, ValueRemovable,
};

impl<Element, State> Capacitary for HashSet<Element, State> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        HashSet::capacity(&self)
    }
}

impl<Element, State> Capacitary for &HashSet<Element, State> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        HashSet::capacity(self)
    }
}

impl<Element, State> Capacitary for &mut HashSet<Element, State> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        HashSet::capacity(self)
    }
}

impl<Element, State> Clearable for &mut HashSet<Element, State> {
    fn clear(self) {
        HashSet::clear(self)
    }
}

impl<Element, State> Emptiable for HashSet<Element, State> {
    fn is_empty(self) -> bool {
        HashSet::is_empty(&self)
    }
}

impl<Element, State> Emptiable for &HashSet<Element, State> {
    fn is_empty(self) -> bool {
        HashSet::is_empty(self)
    }
}

impl<Element, State> Emptiable for &mut HashSet<Element, State> {
    fn is_empty(self) -> bool {
        HashSet::is_empty(self)
    }
}

impl<'a, Element, State> Iterable for &'a HashSet<Element, State> {
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        HashSet::iter(self)
    }
}

impl<'a, Element, State> Iterable for &'a mut HashSet<Element, State> {
    type Output = Iter<'a, Element>;

    fn iter(self) -> Self::Output {
        HashSet::iter(self)
    }
}

impl<Element, State> Lengthsome for HashSet<Element, State> {
    type Length = usize;

    fn len(self) -> Self::Length {
        HashSet::len(&self)
    }
}

impl<Element, State> Lengthsome for &HashSet<Element, State> {
    type Length = usize;

    fn len(self) -> Self::Length {
        HashSet::len(self)
    }
}

impl<Element, State> Lengthsome for &mut HashSet<Element, State> {
    type Length = usize;

    fn len(self) -> Self::Length {
        HashSet::len(self)
    }
}

impl<Element: Eq + Hash, State: BuildHasher> Reservable
    for &mut HashSet<Element, State>
{
    fn reserve(self, additional: Self::Capacity) {
        HashSet::reserve(self, additional)
    }
}

impl<Element: Eq + Hash, State: BuildHasher> TryReservable
    for &mut HashSet<Element, State>
{
    type Error = TryReserveError;

    fn try_reserve(
        self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        HashSet::try_reserve(self, additional)
    }
}

impl<'a, Element: Eq + Hash, State: BuildHasher> ValueContainer
    for &'a HashSet<Element, State>
{
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        HashSet::contains(self, value)
    }
}

impl<'a, Element: Eq + Hash, State: BuildHasher> ValueContainer
    for &'a mut HashSet<Element, State>
{
    type Value = &'a Element;

    fn contains_value(self, value: Self::Value) -> bool {
        HashSet::contains(self, value)
    }
}

impl<Element: Eq + Hash, State: BuildHasher> ValueInsertable
    for &mut HashSet<Element, State>
{
    type Output = bool;
    type Value = Element;

    fn insert_value(self, value: Self::Value) -> Self::Output {
        HashSet::insert(self, value)
    }
}

impl<'a, Element: Hash + Eq, State: BuildHasher> ValueRemovable
    for &'a mut HashSet<Element, State>
{
    type Output = bool;
    type Value = &'a Element;

    fn remove_value(self, value: Self::Value) -> Self::Output {
        HashSet::remove(self, value)
    }
}
