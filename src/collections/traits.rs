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
    fn capacity(&self) -> Self::Capacity;
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
    fn clear(&mut self);
}

pub trait Emptiable {
    /// Checks if collection is empty.
    /// ```
    /// use traiter::collections::Emptiable;
    /// assert!(Emptiable::is_empty(&[0; 0]));
    /// assert!(!Emptiable::is_empty(&[0]));
    /// ```
    fn is_empty(&self) -> bool;
}

pub trait ItemInsertable {
    type Key;
    type Output;
    type Value;

    /// Inserts value into a collection.
    #[cfg_attr(
        feature = "std",
        doc = r##"
```
use traiter::collections::ItemInsertable;
assert_eq!(ItemInsertable::insert_item(&mut vec![10], 0, 20), ());
```
"##
    )]
    fn insert_item(
        &mut self,
        key: Self::Key,
        value: Self::Value,
    ) -> Self::Output;
}

pub trait ItemRemovable<'a> {
    type Output;
    type Key;

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
    fn remove_item(&'a mut self, key: Self::Key) -> Self::Output;
}

pub trait Iterable<'a> {
    type Output: Iterator;

    /// Returns an iterator over elements in a collection.
    /// ```
    /// use traiter::collections::Iterable;
    /// assert_eq!(Iterable::iter(&[0; 0]).next(), None);
    /// ```
    fn iter(&'a self) -> Self::Output
    where
        <<Self as Iterable<'a>>::Output as Iterator>::Item: 'a;
}

pub trait Lengthsome {
    type Length;

    /// Returns number of elements in a collection.
    /// ```
    /// use traiter::collections::Lengthsome;
    /// assert_eq!(Lengthsome::len(&[0; 0]), 0);
    /// assert_eq!(Lengthsome::len(&[0]), 1);
    /// ```
    fn len(&self) -> Self::Length;
}

pub trait MutIterable<'a> {
    type Output: Iterator;

    /// Returns an iterator over mutable elements in a collection.
    /// ```
    /// use traiter::collections::MutIterable;
    /// assert_eq!(MutIterable::iter_mut(&mut [0; 0]).next(), None);
    /// ```
    fn iter_mut(&'a mut self) -> Self::Output
    where
        <<Self as MutIterable<'a>>::Output as Iterator>::Item: 'a;
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
    fn reserve(&mut self, additional: Self::Capacity);
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
        &mut self,
        additional: Self::Capacity,
    ) -> Result<(), Self::Error>;
}

pub trait ValueContainer<'a> {
    type Value;

    /// Checks if collection contains a value.
    /// ```
    /// use traiter::collections::ValueContainer;
    /// assert!(ValueContainer::contains_value(&[0], &0));
    /// assert!(!ValueContainer::contains_value(&[0], &1));
    /// ```
    fn contains_value(&'a self, value: Self::Value) -> bool;
}

pub trait ValueRemovable<'a> {
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
    fn remove_value(&'a mut self, value: Self::Value) -> Self::Output;
}
