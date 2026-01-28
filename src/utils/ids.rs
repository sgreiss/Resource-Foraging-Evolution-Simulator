use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id<T> {
    value: u32,
    _marker: PhantomData<*const T>,
}

impl<T> Id<T> {
    pub fn new(value: u32) -> Self {
        Id {
            value,
            _marker: PhantomData,
        }
    }

    pub fn convert<U>(&self) -> Id<U> {
        Id::new(self.value)
    }

    pub fn raw(self) -> u32 {
        self.value
    }

    pub fn next(&self) -> Id<T> {
        Id::new(self.value + 1)
    }
}

#[derive(Clone, Debug)]
pub struct IdManager {
    latest: u32,
}

impl IdManager {
    pub fn new() -> Self {
        IdManager { latest: 0 }
    }

    pub fn next_id<T>(&mut self) -> Id<T> {
        self.latest += 1;
        Id::new(self.latest)
    }
}
