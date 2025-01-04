use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;
use surrealdb::sql::{Thing, Value};

pub trait SurrealId: From<Value> {
    fn id(&self) -> Thing;
}

#[deprecated(
    note = "This crate is deprecated. Please use surreal_derive and surreal_devl instead."
)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Link<T>
where
    T: SurrealId,
{
    Id(Thing),
    Record(T),
}

impl<T> PartialEq for Link<T>
where
    T: SurrealId,
{
    fn eq(&self, other: &Self) -> bool {
        other.id() == self.id()
    }
}

impl<T> Link<T>
where
    T: SurrealId,
{
    pub fn id(&self) -> Thing {
        match self {
            Self::Id(id) => id.clone(),
            Self::Record(r) => r.id(),
        }
    }

    pub fn record(self) -> T {
        match self {
            Self::Id(_) => {
                panic!("Expected a record got an id")
            }
            Self::Record(r) => r,
        }
    }
}

impl<T> Into<Thing> for Link<T>
where
    T: SurrealId,
{
    fn into(self) -> Thing {
        self.id()
    }
}

impl<T> Into<Thing> for &Link<T>
where
    T: SurrealId,
{
    fn into(self) -> Thing {
        self.id().clone()
    }
}

impl<T> Into<Value> for Link<T>
where
    T: SurrealId,
{
    fn into(self) -> Value {
        Value::Thing(self.id())
    }
}

impl<T> From<Value> for Link<T>
where
    T: SurrealId,
{
    fn from(value: Value) -> Self {
        match value {
            Value::Thing(id) => Self::Id(id),
            Value::Object(obj) => Self::Record(Value::Object(obj).into()),
            _ => panic!("Expected id or object"),
        }
    }
}

impl<T> Deref for Link<T>
where
    T: SurrealId,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Link::Id(_) => {
                panic!("The link can not be deref, it must be Link::Record(T) to be deref")
            }
            Link::Record(r) => &r,
        }
    }
}

pub trait NewLink<T, P>
where
    T: SurrealId,
{
    fn new(params: P) -> Link<T>;
}
