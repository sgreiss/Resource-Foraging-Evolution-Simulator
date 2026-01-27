use std::{{collections::HashMap, marker::PhantomData, any::TypeId}};

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
}

#[derive(Clone, Debug)]
pub struct IdManager {
    latest: HashMap<TypeId, u32>,
}

impl IdManager {
    pub fn new() -> Self {
        IdManager {
            latest: HashMap::new(),
        }
    }

    pub fn next_id<T : 'static>(&mut self) -> Id<T> {
        let type_id = TypeId::of::<T>();
        let counter = self.latest.entry(type_id).or_insert(0);
        let id = Id::new(*counter);
        *counter += 1;
        id
    }
}
