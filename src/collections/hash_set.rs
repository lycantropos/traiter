use std::collections::hash_set::Iter;
use std::collections::{HashSet, TryReserveError};
use std::hash::{BuildHasher, Hash};

use super::traits::{
    Capacitary, Clearable, Container, Emptiable, Iterable, Lengthsome,
    Removable, Reservable, TryReservable,
};

impl<Element, State> Capacitary for HashSet<Element, State> {
    type Capacity = usize;

    fn capacity(&self) -> Self::Capacity {
        HashSet::capacity(self)
    }
}

impl<Element, State> Clearable for HashSet<Element, State> {
    fn clear(&mut self) {
        HashSet::clear(self)
    }
}

impl<Element: Eq + Hash, State: BuildHasher> Container
    for HashSet<Element, State>
{
    type Value = Element;

    fn contains(&self, value: &Self::Value) -> bool {
        HashSet::contains(self, value)
    }
}

impl<Element, State> Emptiable for HashSet<Element, State> {
    fn is_empty(&self) -> bool {
        HashSet::is_empty(self)
    }
}

impl<'a, Element: 'a, State> Iterable<'a> for HashSet<Element, State> {
    type Output = Iter<'a, Element>;

    fn iter(&'a self) -> Self::Output {
        HashSet::iter(self)
    }
}

impl<Element, State> Lengthsome for HashSet<Element, State> {
    type Length = usize;

    fn len(&self) -> Self::Length {
        HashSet::len(self)
    }
}

impl<'a, Element: 'a + Hash + Eq, State: BuildHasher> Removable<'a>
    for HashSet<Element, State>
{
    type Output = bool;
    type Value = &'a Element;

    fn remove(&'a mut self, value: Self::Value) -> Self::Output {
        HashSet::remove(self, value)
    }
}

impl<Element: Eq + Hash, State: BuildHasher> Reservable
    for HashSet<Element, State>
{
    fn reserve(&mut self, additional: Self::Capacity) {
        HashSet::reserve(self, additional)
    }
}

impl<Element: Eq + Hash, State: BuildHasher> TryReservable
    for HashSet<Element, State>
{
    type Error = TryReserveError;

    fn try_reserve(
        &mut self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        HashSet::try_reserve(self, additional)
    }
}
