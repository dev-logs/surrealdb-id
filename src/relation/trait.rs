use surrealdb::sql::Thing;

use crate::relation::Relation;

pub trait IntoRelation<I, O>
where
    Self: Sized + Into<Thing>,
    I: Sized + Into<Thing>,
    O: Sized + Into<Thing>,
{
    fn relate(self, i: I, o: O) -> Relation<I, Self, O>;
}

impl<I, R, O> IntoRelation<I, O> for R
where
    R: Sized + Into<Thing>,
    I: Sized + Into<Thing>,
    O: Sized + Into<Thing>,
{
    fn relate(self, i: I, o: O) -> Relation<I, Self, O> {
        Relation {
            r#in: Some(i),
            out: Some(o),
            relation: self,
        }
    }
}

impl<I, R, O> Into<Thing> for Relation<I, R, O>
where
    R: Sized + Into<Thing>,
    I: Sized + Into<Thing>,
    O: Sized + Into<Thing>,
{
    fn into(self) -> Thing {
        self.relation.into()
    }
}

impl<I, R, O> Into<Thing> for &Relation<I, R, O>
where
    R: Sized + Into<Thing> + Clone,
    I: Sized + Into<Thing>,
    O: Sized + Into<Thing>,
{
    fn into(self) -> Thing {
        (&self.relation).clone().into()
    }
}
