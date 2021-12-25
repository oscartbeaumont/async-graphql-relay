//! Relay support for [async-graphql](https://github.com/async-graphql/async-graphql).
//! Check out [the example application](https://github.com/oscartbeaumont/async-graphql-relay/tree/main/example) to get started.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::{any::Any, fmt, marker::PhantomData, str::FromStr};

use async_graphql::{Error, InputValueResult, Scalar, ScalarType, Value};

pub use async_graphql_relay_derive::*;
use async_trait::async_trait;
use uuid::Uuid;

#[doc(hidden)]
pub use async_trait::async_trait as _async_trait;

/// RelayNodeInterface is a trait implemented by the GraphQL interface enum to implement the fetch_node method.
/// You should refer to the 'RelayInterface' macro which is the recommended way to implement this trait.
#[async_trait]
pub trait RelayNodeInterface
where
    Self: Sized,
{
    /// fetch_node takes in a RelayContext and a generic relay ID and will return a Node interface with the requested object.
    /// This function is used to implement the 'node' query required by the Relay server specification for easily refetching an entity in the GraphQL schema.
    async fn fetch_node(ctx: RelayContext, relay_id: String) -> Result<Self, Error>;
}

/// RelayNodeStruct is a trait implemented by the GraphQL Object to ensure each Object has a globally unique ID.
/// You should refer to the 'RelayNodeObject' macro which is the recommended way to implement this trait.
/// You MUST ensure the ID_SUFFIX is unique for each object for issue will occur.
pub trait RelayNodeStruct {
    /// ID_SUFFIX is the suffix appended to the nodes ID to create the relay ID.
    /// This MUST be unique for each type in the system.
    const ID_SUFFIX: &'static str;
}

/// RelayNode is a trait implemented on the GraphQL Object to define how it should be fetched.
/// This is used by the 'node' query so that the object can be refetched.
#[async_trait]
pub trait RelayNode: RelayNodeStruct {
    /// TNode is the type of the Node interface. This should point the enum with the 'RelayInterface' macro.
    type TNode: RelayNodeInterface;

    /// get is a method defines by the user to refetch an object of a particular type.
    /// The context can be used to share a database connection or other required context to facilitate the refetch.
    async fn get(ctx: RelayContext, id: RelayNodeID<Self>) -> Result<Option<Self::TNode>, Error>;
}

/// RelayNodeID is a wrapper around a UUID with the use of the 'RelayNodeStruct' trait to ensure each object has a globally unique ID.
#[derive(Clone, PartialEq, Eq)]
pub struct RelayNodeID<T: RelayNode + ?Sized>(Uuid, PhantomData<T>);

impl<T: RelayNode> RelayNodeID<T> {
    /// new creates a new RelayNodeID from a UUID string and a type specified as a generic.
    pub fn new(uuid: Uuid) -> Self {
        RelayNodeID(uuid, PhantomData)
    }

    /// new_from_relay_id takes in a generic relay ID and converts it into a RelayNodeID.
    pub fn new_from_relay_id(relay_id: String) -> Result<Self, Error> {
        if relay_id.len() < 32 {
            return Err(Error::new("Invalid id provided to node query!"));
        }
        let (id, _) = relay_id.split_at(32);
        let uuid = Uuid::parse_str(&id)
            .map_err(|_err| Error::new("Invalid id provided to node query!"))?;
        Ok(RelayNodeID(uuid, PhantomData))
    }

    /// new_from_str is a wrapper around 'Uuid::from_str' to create a new RelayNodeID from a UUIDv4 string.
    pub fn new_from_str(uuid: &str) -> Result<Self, uuid::Error> {
        Ok(Self::new(Uuid::from_str(uuid)?))
    }

    /// to_uuid will convert the RelayNodeID into a normal UUID for use in DB queries or internal processing.
    /// The Uuid from this function is NOT globally unique!
    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl<T: RelayNode> From<&RelayNodeID<T>> for String {
    fn from(id: &RelayNodeID<T>) -> Self {
        format!("{}{}", id.0.to_simple().to_string(), T::ID_SUFFIX)
    }
}

impl<T: RelayNode> ToString for RelayNodeID<T> {
    fn to_string(&self) -> String {
        String::from(self)
    }
}

impl<T: RelayNode> fmt::Debug for RelayNodeID<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RelayNodeID").field(&self.0).finish()
    }
}

#[Scalar]
impl<T: RelayNode + Send + Sync> ScalarType for RelayNodeID<T> {
    fn parse(_value: Value) -> InputValueResult<Self> {
        unimplemented!();
    }

    fn to_value(&self) -> Value {
        Value::String(String::from(self))
    }
}

/// RelayContext allows context to be parsed to the `get` handler to facilitate refetching of objects.
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

#[cfg(feature = "with-sea-orm")]
impl<T: RelayNode> From<RelayNodeID<T>> for sea_orm::Value {
    fn from(source: RelayNodeID<T>) -> Self {
        sea_orm::Value::Uuid(Some(Box::new(source.to_uuid())))
    }
}

#[cfg(feature = "with-sea-orm")]
impl<T: RelayNode> sea_orm::TryGetable for RelayNodeID<T> {
    fn try_get(
        res: &sea_orm::QueryResult,
        pre: &str,
        col: &str,
    ) -> Result<Self, sea_orm::TryGetError> {
        let val: Uuid = res.try_get(pre, col).map_err(sea_orm::TryGetError::DbErr)?;
        Ok(RelayNodeID::<T>::new(val))
    }
}

#[cfg(feature = "with-sea-orm")]
impl<T: RelayNode> sea_orm::sea_query::Nullable for RelayNodeID<T> {
    fn null() -> sea_orm::Value {
        sea_orm::Value::Uuid(None)
    }
}

#[cfg(feature = "with-sea-orm")]
impl<T: RelayNode> sea_orm::sea_query::ValueType for RelayNodeID<T> {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        match v {
            sea_orm::Value::Uuid(Some(x)) => Ok(RelayNodeID::<T>::new(*x)),
            _ => Err(sea_orm::sea_query::ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Uuid).to_owned()
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        sea_orm::sea_query::ColumnType::Uuid
    }
}

#[cfg(feature = "with-sea-orm")]
impl<T: RelayNode> sea_orm::TryFromU64 for RelayNodeID<T> {
    fn try_from_u64(_: u64) -> Result<Self, sea_orm::DbErr> {
        Err(sea_orm::DbErr::Exec(format!(
            "{} cannot be converted from u64",
            std::any::type_name::<T>()
        )))
    }
}

// TODO: Unit tests
