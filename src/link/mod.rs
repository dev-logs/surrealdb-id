use std::collections::BTreeMap;
use std::ops::Deref;
use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::{RecordId};
use surrealdb::sql::{Value};

/// A relation between table in surrealdb
/// It could be either link in case the query is not perform fetch
/// If fetch is perform the link will be None instead the record will be Some(T)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Link<T> where T: Sized + Into<RecordId> {
    Id(RecordId),
    Record(T),
}

impl<T> Link<T> where T: Clone + Sized + Into<RecordId> {
    pub fn id(&self) -> RecordId {
        match self {
            Self::Id(id) => id.clone(),
            Self::Record(r) => r.clone().into()
        }
    }
}

impl<T> Into<RecordId> for Link<T> where T: Clone + Sized + Into<RecordId> {
    fn into(self) -> RecordId {
        self.id().clone()
    }
}

impl<T> From<Link<T>> for Value where T: Clone + Sized + Into<RecordId> {
    fn from(value: Link<T>) -> Self {
        Value::Thing(value.id())
    }
}

impl<T> Deref for Link<T> where T: Clone + Sized + Into<RecordId> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Link::Id(_) => panic!("The link can not be deref, it must be Link::Record(T) to be deref"),
            Link::Record(r) => {&r}
        }
    }
}

impl <T, R> From<R> for Link<T> where
    surrealdb::sql::Thing: From<R>,
    T: Sized + Into<RecordId> {
    fn from(value: R) -> Self {
        Self::Id(RecordId::from(value))
    }
}
