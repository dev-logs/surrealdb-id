use surrealdb::opt::RecordId;
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
            r#in: Some(i),
            out: Some(o),
            relation: self
        }
    }
}

impl<I, R, O> Into<RecordId> for Relation<I, R, O> where
    R: Sized + Into<RecordId>,
    I: Sized + Into<RecordId>,
    O: Sized + Into<RecordId>
{
    fn into(self) -> RecordId {
        self.relation.into()
    }
}

impl<I, R, O> Into<RecordId> for &Relation<I, R, O> where
    R: Sized + Into<RecordId> + Clone,
    I: Sized + Into<RecordId>,
    O: Sized + Into<RecordId>
{
    fn into(self) -> RecordId {
        (&self.relation).clone().into()
    }
}
