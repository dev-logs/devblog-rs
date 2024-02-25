use std::fmt::Debug;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::relation::relation::Relation;

pub trait IntoRelation<I, O> where
    Self: Clone,
    I: Clone + Sized + Into<RecordId>,
    O: Clone + Sized + Into<RecordId>
{
    fn into_relation(&self, i: AdaptiveRelation<I>, o: AdaptiveRelation<O>) -> Relation<Self, I, O>;
}

impl<T, I, O> IntoRelation<I, O> for T where
T: Debug + Clone,
I: Clone + Sized + Into<RecordId>,
O: Clone + Sized + Into<RecordId>
{
    fn into_relation(&self, i: AdaptiveRelation<I>, o: AdaptiveRelation<O>) -> Relation<Self, I, O> {
        Relation {
            r#in: i,
            out: o,
            relation: self.clone(),
        }
    }
}
