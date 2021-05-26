use std::any::Any;

pub use async_graphql_relay_derive::*;

pub struct RelayContext(Box<dyn Any + Sync + Send>);

impl RelayContext {
    pub fn new<T: Any + Sync + Send>(data: T) -> Self {
        Self(Box::new(data))
    }

    pub fn nil() -> Self {
        let nil: Option<()> = None;
        Self(Box::new(nil))
    }

    pub fn get<T: Any + Sync + Send>(&self) -> Option<&T> {
        match self.0.downcast_ref::<T>() {
            Some(v) => Some(v),
            _ => None,
        }
    }
}
