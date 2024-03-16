use std::fmt::Debug;
use surrealdb::opt::RecordId;
use crate::link::Link;
use crate::relation::Relation;

pub trait IntoRelation<I, O> where
    Self: Sized + Into<RecordId>,
    I: Sized + Into<RecordId>,
    O: Sized + Into<RecordId>
{
    fn relate(self, i: I, o: O) -> Relation<I, Self, O>;
}

impl<I, R, O> IntoRelation<I, O> for R where
    R: Sized + Into<RecordId>,
    I: Sized + Into<RecordId>,
    O: Sized + Into<RecordId>
{
    fn relate(self, i: I, o: O) -> Relation<I, Self, O> {
        Relation {
            r#in: i,
            out: o,
            relation: self
        }
    }
}
