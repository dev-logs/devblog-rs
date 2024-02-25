use std::ops::Deref;
use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Relation<T, I, O> where
    I: Clone + Sized + Into<RecordId>,
    O: Clone + Sized + Into<RecordId>,
    T: Clone
{
    pub r#in: AdaptiveRelation<I>,
    pub out: AdaptiveRelation<O>,
    #[serde(flatten)]
    pub relation: T
}

impl<T, I, O> Deref for Relation<T, I, O> where
    I: Clone + Sized + Into<RecordId>,
    O: Clone + Sized + Into<RecordId>,
    T: Clone
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.relation
    }
}
