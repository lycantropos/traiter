pub trait Capacitary {
    type Capacity;

    /// Returns capacity of a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::collections::Capacitary;
let collection = Vec::<i8>::with_capacity(10);
assert_eq!(Capacitary::capacity(&collection), 10);
```
"##
    )]
    fn capacity(self) -> Self::Capacity;
}

pub trait Clearable {
    /// Removes all elements from a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::collections::{Clearable, Emptiable};
let mut collection = vec![0];
assert!(!Emptiable::is_empty(&collection));
Clearable::clear(&mut collection);
assert!(Emptiable::is_empty(&collection));
```
"##
    )]
    fn clear(self);
}

pub trait Emptiable {
    /// Checks if collection is empty.
    /// ```
    /// use traiter::collections::Emptiable;
    /// assert!(Emptiable::is_empty(&[0; 0]));
    /// assert!(!Emptiable::is_empty(&[0]));
    /// ```
    fn is_empty(self) -> bool;
}

pub trait ItemInsertable {
    type Key;
    type Output;
    type Value;

    /// Inserts item into a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::collections::ItemInsertable;
assert_eq!(ItemInsertable::insert_item(&mut vec![10], 0, 20), ());
```
"##
    )]
    fn insert_item(self, key: Self::Key, value: Self::Value) -> Self::Output;
}

pub trait ItemRemovable {
    type Key;
    type Output;

    /// Removes item from a collection by key.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::collections::ItemRemovable;
assert_eq!(ItemRemovable::remove_item(&mut vec![10], 0), 10);
```
"##
    )]
    fn remove_item(self, key: Self::Key) -> Self::Output;
}

pub trait Iterable {
    type Item;
    type Output: Iterator<Item = Self::Item>;

    /// Returns an iterator over elements in a collection.
    /// ```
    /// use traiter::collections::Iterable;
    /// assert_eq!(Iterable::iter(&[0; 0]).next(), None);
    /// ```
    fn iter(self) -> Self::Output;
}

pub trait KeyContainer {
    type Key;

    /// Checks if collection contains a key.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use std::collections::HashMap;
use traiter::collections::{KeyContainer, ItemInsertable};
let mut collection = HashMap::new();
assert!(!KeyContainer::contains_key(&collection, &10));
assert_eq!(ItemInsertable::insert_item(&mut collection, 10, 20), None);
assert!(KeyContainer::contains_key(&collection, &10));
```
"##
    )]
    fn contains_key(self, key: Self::Key) -> bool;
}

pub trait Lengthsome {
    type Length;

    /// Returns number of elements in a collection.
    /// ```
    /// use traiter::collections::Lengthsome;
    /// assert_eq!(Lengthsome::len([0; 0]), 0);
    /// assert_eq!(Lengthsome::len([0]), 1);
    /// ```
    fn len(self) -> Self::Length;
}

pub trait MutablyIterable {
    type Item;
    type Output: Iterator<Item = Self::Item>;

    /// Returns an iterator over mutable elements in a collection.
    /// ```
    /// use traiter::collections::MutablyIterable;
    /// assert_eq!(MutablyIterable::iter_mut(&mut [0; 0]).next(), None);
    /// ```
    fn iter_mut(self) -> Self::Output;
}

pub trait Reservable: Capacitary {
    /// Reserves capacity for at least given number of elements for a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::collections::{Capacitary, Reservable};
let mut collection = Vec::<i8>::with_capacity(10);
assert_eq!(Capacitary::capacity(&collection), 10);
Reservable::reserve(&mut collection, 20);
assert_eq!(Capacitary::capacity(&collection), 20);
```
"##
    )]
    fn reserve(self, additional: Self::Capacity);
}

pub trait TryReservable: Capacitary {
    type Error;

    /// Tries to reserve capacity for at least given number of elements for a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::collections::{Capacitary, Reservable};
let mut collection = Vec::<i8>::with_capacity(10);
assert_eq!(Capacitary::capacity(&collection), 10);
Reservable::reserve(&mut collection, 20);
assert_eq!(Capacitary::capacity(&collection), 20);
```
"##
    )]
    fn try_reserve(
        self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error>;
}

pub trait ValueContainer {
    type Value;

    /// Checks if collection contains a value.
    /// ```
    /// use traiter::collections::ValueContainer;
    /// assert!(ValueContainer::contains_value(&[0], &0));
    /// assert!(!ValueContainer::contains_value(&[0], &1));
    /// ```
    fn contains_value(self, value: Self::Value) -> bool;
}

pub trait ValueInsertable {
    type Output;
    type Value;

    /// Inserts value into a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use std::collections::HashSet;
use traiter::collections::{ValueContainer, ValueInsertable};
let mut collection = HashSet::new();
assert!(!ValueContainer::contains_value(&collection, &10));
assert!(ValueInsertable::insert_value(&mut collection, 10));
assert!(ValueContainer::contains_value(&collection, &10));
```
"##
    )]
    fn insert_value(self, value: Self::Value) -> Self::Output;
}

pub trait ValueRemovable {
    type Output;
    type Value;

    /// Removes value from a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use std::collections::HashSet;
use traiter::collections::ValueRemovable;
let mut collection = HashSet::new();
collection.insert(10);
assert!(ValueRemovable::remove_value(&mut collection, &10));
assert!(!ValueRemovable::remove_value(&mut collection, &10));
```
"##
    )]
    fn remove_value(self, value: Self::Value) -> Self::Output;
}
