pub mod r#trait;

use serde::{Deserialize, Serialize};
use std::ops::Deref;
use surrealdb::sql::Thing;

use crate::link::Link;

/// https://crates.io/crates/surreal_derive_plus
/// T: is the relation content
/// I: in relation
/// O: out relation
/// Example:
/// ```
/// let marry = User::new("marry");
/// let john = User::new("tiendang");
/// let married = Marrying::new("2024/01/01");
/// let relation = married.relate(john, marry);
/// // RELATE user:join -> married -> user:marry SET date = "2024/01/01"
/// ```
#[deprecated(
    note = "This crate is deprecated. Please use surreal_derive and surreal_devl instead."
)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relation<I, R, O>
where
    R: Sized + Into<Thing>,
    I: Sized + Into<Thing>,
    O: Sized + Into<Thing>,
{
    pub r#in: Option<I>,
    pub out: Option<O>,
    #[serde(flatten)]
    pub relation: R,
}

impl<I, R, O> Deref for Relation<I, R, O>
where
    R: Sized + Into<Thing>,
    I: Into<Thing>,
    O: Sized + Into<Thing>,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.relation
    }
}

/// IdRelation will use RecordId for both r#in and out
/// ```
///use serde_derive::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Serialize, Deserialize)]
///struct Friend {}
///
/// let relation: IdRelation<Friend> = db.query("RELATE user:devlog -> friend -> user:tien").await?.take(0);
/// ```
pub type IdRelation<T> = Relation<Thing, T, Thing>;

/// LinkRelation will support type for r#in and out
/// ```
///use serde_derive::{Deserialize, Serialize};
///use surrealdb_id::relation::LinkRelation;
///
///#[derive(Clone, Debug, Serialize, Deserialize)]
///struct Friend {}
///
///let relation: LinkRelation<User, Friend, User> = db.query("RELATE user:devlog -> friend -> user:tien").await?.take(0);
/// ```
pub type LinkRelation<I, R, O> = Relation<Link<I>, R, Link<O>>;
