use collections::Removable;
use std::collections::hash_map::{Iter, IterMut};
use std::collections::{HashMap, TryReserveError};
use std::hash::{BuildHasher, Hash};

use super::traits::{
    Capacitary, Clearable, Emptiable, Iterable, Lengthsome, MutIterable,
    Reservable, TryReservable,
};

impl<Key, Value, State> Capacitary for HashMap<Key, Value, State> {
    type Capacity = usize;

    fn capacity(&self) -> Self::Capacity {
        HashMap::capacity(self)
    }
}

impl<Key, Value, State> Clearable for HashMap<Key, Value, State> {
    fn clear(&mut self) {
        HashMap::clear(self)
    }
}

impl<Key, Value, State> Emptiable for HashMap<Key, Value, State> {
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<'a, Key: 'a, Value: 'a, State> Iterable<'a>
    for HashMap<Key, Value, State>
{
    type Output = Iter<'a, Key, Value>;

    fn iter(&'a self) -> Self::Output {
        HashMap::iter(self)
    }
}

impl<Key, Value, State> Lengthsome for HashMap<Key, Value, State> {
    type Length = usize;

    fn len(&self) -> Self::Length {
        HashMap::len(self)
    }
}

impl<'a, Key: 'a + Eq + Hash, Value, State: BuildHasher> Removable<'a>
    for HashMap<Key, Value, State>
{
    type Output = Option<Value>;
    type Value = &'a Key;

    fn remove(&'a mut self, value: Self::Value) -> Self::Output {
        HashMap::remove(self, value)
    }
}

impl<Key: Eq + Hash, Value, State: BuildHasher> Reservable
    for HashMap<Key, Value, State>
{
    fn reserve(&mut self, additional: Self::Capacity) {
        HashMap::reserve(self, additional)
    }
}

impl<Key: Eq + Hash, Value, State: BuildHasher> TryReservable
    for HashMap<Key, Value, State>
{
    type Error = TryReserveError;

    fn try_reserve(
        &mut self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        HashMap::try_reserve(self, additional)
    }
}

impl<'a, Key: 'a, Value: 'a, State> MutIterable<'a>
    for HashMap<Key, Value, State>
{
    type Output = IterMut<'a, Key, Value>;

    fn iter_mut(&'a mut self) -> Self::Output {
        HashMap::iter_mut(self)
    }
}
