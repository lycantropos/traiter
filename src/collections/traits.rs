pub trait Capacitary {
    type Capacity;

    fn capacity(&self) -> Self::Capacity;
}

pub trait Container<Value> {
    fn contains(&self, value: Value) -> bool;
}

pub trait Emptiable {
    fn is_empty(&self) -> bool;
}

pub trait Lengthsome {
    type Length;

    fn len(&self) -> Self::Length;
}
