pub mod r#trait;

use std::ops::Deref;
use serde::{Deserialize, Serialize};

use surrealdb::opt::RecordId;
use crate::link::Link;
use crate::relation::r#trait::IntoRelation;

/// This class represent for feature relation of Surrealdb
/// https://docs.surrealdb.com/docs/surrealql/statements/relate
/// T: is the relation content
/// I: in relation
/// O: out relation
/// Example:
/// let marry = User::new("marry");
/// let john = User::new("tiendang");
/// let married = Marrying::new("2024/01/01");
/// // RELATE user:join -> married -> user:marry SET date = "2024/01/01"
/// married.into_relation(john, marry);

#[derive(Serialize, Deserialize)]
pub struct Relation<I, R, O> where
    R: Sized + Into<RecordId>,
    I: Sized + Into<RecordId>,
    O: Sized + Into<RecordId>
{
    pub r#in: I,
    pub out: O,
    #[serde(flatten)]
    pub relation: R
}

impl<I, R, O> Deref for Relation<I, R, O> where
    R: Sized + Into<RecordId>,
    I: Into<RecordId>,
    O: Sized + Into<RecordId>
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.relation
    }
}

pub type IdRelation<T> = Relation<RecordId, T, RecordId>;

pub type LinkRelation<I, R, O> = Relation<Link<I>, R, Link<O>>;
