use std::collections::hash_map::{Iter, IterMut};
use std::collections::{HashMap, TryReserveError};
use std::hash::{BuildHasher, Hash};

use super::traits::{
    Capacitary, Clearable, Emptiable, ItemInsertable, ItemRemovable, Iterable,
    KeyContainer, Lengthsome, MutablyIterable, Reservable, TryReservable,
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

impl<'a, Key: 'a + Eq + Hash, Value, State: BuildHasher> ItemInsertable
    for HashMap<Key, Value, State>
{
    type Key = Key;
    type Output = Option<Value>;
    type Value = Value;

    fn insert_item(
        &mut self,
        key: Self::Key,
        value: Self::Value,
    ) -> Self::Output {
        HashMap::insert(self, key, value)
    }
}

impl<'a, Key: 'a + Eq + Hash, Value, State: BuildHasher> ItemRemovable<'a>
    for HashMap<Key, Value, State>
{
    type Output = Option<Value>;
    type Key = &'a Key;

    fn remove_item(&'a mut self, key: Self::Key) -> Self::Output {
        HashMap::remove(self, key)
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

impl<'a, Key: 'a + Eq + Hash, Value, State: BuildHasher> KeyContainer<'a>
    for HashMap<Key, Value, State>
{
    type Key = &'a Key;

    fn contains_key(&'a self, key: Self::Key) -> bool {
        HashMap::contains_key(self, key)
    }
}

impl<Key, Value, State> Lengthsome for HashMap<Key, Value, State> {
    type Length = usize;

    fn len(&self) -> Self::Length {
        HashMap::len(self)
    }
}

impl<'a, Key: 'a, Value: 'a, State> MutablyIterable<'a>
    for HashMap<Key, Value, State>
{
    type Output = IterMut<'a, Key, Value>;

    fn iter_mut(&'a mut self) -> Self::Output {
        HashMap::iter_mut(self)
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
