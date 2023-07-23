use std::collections::hash_map::{Iter, IterMut};
use std::collections::{HashMap, TryReserveError};
use std::hash::{BuildHasher, Hash};

use super::traits::{
    Capacitary, Clearable, Emptiable, ItemInsertable, ItemRemovable, Iterable,
    KeyContainer, Lengthsome, MutablyIterable, Reservable, TryReservable,
};

impl<Key, Value, State> Capacitary for HashMap<Key, Value, State> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        HashMap::capacity(&self)
    }
}

impl<Key, Value, State> Capacitary for &HashMap<Key, Value, State> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        HashMap::capacity(self)
    }
}

impl<Key, Value, State> Capacitary for &mut HashMap<Key, Value, State> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        HashMap::capacity(self)
    }
}

impl<Key, Value, State> Clearable for &mut HashMap<Key, Value, State> {
    fn clear(self) {
        HashMap::clear(self)
    }
}

impl<Key, Value, State> Emptiable for HashMap<Key, Value, State> {
    fn is_empty(self) -> bool {
        HashMap::is_empty(&self)
    }
}

impl<Key, Value, State> Emptiable for &HashMap<Key, Value, State> {
    fn is_empty(self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<Key, Value, State> Emptiable for &mut HashMap<Key, Value, State> {
    fn is_empty(self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<Key: Eq + Hash, Value, State: BuildHasher> ItemInsertable
    for &mut HashMap<Key, Value, State>
{
    type Key = Key;
    type Output = Option<Value>;
    type Value = Value;

    fn insert_item(self, key: Self::Key, value: Self::Value) -> Self::Output {
        HashMap::insert(self, key, value)
    }
}

impl<'a, Key: Eq + Hash, Value, State: BuildHasher> ItemRemovable
    for &'a mut HashMap<Key, Value, State>
{
    type Key = &'a Key;
    type Output = Option<Value>;

    fn remove_item(self, key: Self::Key) -> Self::Output {
        HashMap::remove(self, key)
    }
}

impl<'a, Key, Value, State> Iterable for &'a HashMap<Key, Value, State> {
    type Item = (&'a Key, &'a Value);
    type Output = Iter<'a, Key, Value>;

    fn iter(self) -> Self::Output {
        HashMap::iter(self)
    }
}

impl<'a, Key, Value, State> Iterable for &'a mut HashMap<Key, Value, State> {
    type Item = (&'a Key, &'a Value);
    type Output = Iter<'a, Key, Value>;

    fn iter(self) -> Self::Output {
        HashMap::iter(self)
    }
}

impl<'a, Key: Eq + Hash, Value, State: BuildHasher> KeyContainer
    for &'a HashMap<Key, Value, State>
{
    type Key = &'a Key;

    fn contains_key(self, key: Self::Key) -> bool {
        HashMap::contains_key(self, key)
    }
}

impl<'a, Key: Eq + Hash, Value, State: BuildHasher> KeyContainer
    for &'a mut HashMap<Key, Value, State>
{
    type Key = &'a Key;

    fn contains_key(self, key: Self::Key) -> bool {
        HashMap::contains_key(self, key)
    }
}

impl<Key, Value, State> Lengthsome for HashMap<Key, Value, State> {
    type Length = usize;

    fn len(self) -> Self::Length {
        HashMap::len(&self)
    }
}

impl<Key, Value, State> Lengthsome for &HashMap<Key, Value, State> {
    type Length = usize;

    fn len(self) -> Self::Length {
        HashMap::len(self)
    }
}

impl<Key, Value, State> Lengthsome for &mut HashMap<Key, Value, State> {
    type Length = usize;

    fn len(self) -> Self::Length {
        HashMap::len(self)
    }
}

impl<'a, Key, Value, State> MutablyIterable
    for &'a mut HashMap<Key, Value, State>
{
    type Item = (&'a Key, &'a mut Value);
    type Output = IterMut<'a, Key, Value>;

    fn iter_mut(self) -> Self::Output {
        HashMap::iter_mut(self)
    }
}

impl<Key: Eq + Hash, Value, State: BuildHasher> Reservable
    for &mut HashMap<Key, Value, State>
{
    fn reserve(self, additional: Self::Capacity) {
        HashMap::reserve(self, additional)
    }
}

impl<Key: Eq + Hash, Value, State: BuildHasher> TryReservable
    for &mut HashMap<Key, Value, State>
{
    type Error = TryReserveError;

    fn try_reserve(
        self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error> {
        HashMap::try_reserve(self, additional)
    }
}
