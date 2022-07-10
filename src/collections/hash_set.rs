use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

use super::traits::{Capacitary, Clearable, Container, Emptiable, Lengthsome};

impl<Element> Capacitary for HashSet<Element> {
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

impl<Element: Eq + Hash, State: BuildHasher> Container<&Element> for HashSet<Element, State> {
    fn contains(&self, value: &Element) -> bool {
        HashSet::contains(self, value)
    }
}

impl<Element, State> Emptiable for HashSet<Element, State> {
    fn is_empty(&self) -> bool {
        HashSet::is_empty(self)
    }
}

impl<Element, State> Lengthsome for HashSet<Element, State> {
    type Length = usize;

    fn len(&self) -> Self::Length {
        HashSet::len(self)
    }
}
