use std::collections::HashMap;

use super::traits::{Capacitary, Container, Emptiable, Lengthsome};
use std::hash::{BuildHasher, Hash};

impl<Key: Eq + Hash, Value, State: BuildHasher> Container<&Key> for &HashMap<Key, Value, State> {
    fn contains(self, value: &Key) -> bool {
        HashMap::contains_key(self, value)
    }
}

impl<Key, Value, State> Capacitary for &HashMap<Key, Value, State> {
    type Capacity = usize;

    fn capacity(self) -> Self::Capacity {
        HashMap::capacity(self)
    }
}

impl<Key, Value, State> Emptiable for &HashMap<Key, Value, State> {
    fn is_empty(self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<Key, Value, State> Lengthsome for &HashMap<Key, Value, State> {
    type Length = usize;

    fn len(self) -> Self::Length {
        HashMap::len(self)
    }
}
