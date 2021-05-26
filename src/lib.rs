//! Relay support for [async-graphql](https://github.com/async-graphql/async-graphql).
//! Check out [the example application](https://github.com/oscartbeaumont/async-graphql-relay/tree/main/example) to get started.

use std::any::Any;

pub use async_graphql_relay_derive::*;

/// RelayContext allows context to be parsed to the `get` handlers.
/// This is designed for parsing the Database connection but could be used for any global state.
pub struct RelayContext(Box<dyn Any + Sync + Send>);

impl RelayContext {
    /// Create a new context which stores a piece of data.
    pub fn new<T: Any + Sync + Send>(data: T) -> Self {
        Self(Box::new(data))
    }

    /// Create a new empty context. This can be used if you have no data to put in the context.
    pub fn nil() -> Self {
        let nil: Option<()> = None;
        Self(Box::new(nil))
    }

    /// Get a pointer to the data stored in the context if it can be found.
    pub fn get<T: Any + Sync + Send>(&self) -> Option<&T> {
        match self.0.downcast_ref::<T>() {
            Some(v) => Some(v),
            _ => None,
        }
    }
}
