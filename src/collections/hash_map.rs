use std::collections::hash_map::Iter;
use std::collections::{HashMap, TryReserveError};
use std::hash::{BuildHasher, Hash};

use super::traits::{
    Capacitary, Clearable, Container, Emptiable, Iterable, Lengthsome,
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

impl<Key: Eq + Hash, Value, State: BuildHasher> Container<&Key>
    for HashMap<Key, Value, State>
{
    fn contains(&self, value: &Key) -> bool {
        HashMap::contains_key(self, value)
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
