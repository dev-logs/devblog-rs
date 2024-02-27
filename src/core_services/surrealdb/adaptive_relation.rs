use std::ops::Deref;
use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb::sql::Value;

/// A relation between table in surrealdb
/// It could be either id in case the query is not perform fetch
/// If fetch is perform the id will be None instead the record will be Some(T)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum AdaptiveRelation<T> where T: Clone + Sized + Into<RecordId> {
    Id(RecordId),
    Record(T),
}

impl<T> AdaptiveRelation<T> where T: Clone + Sized + Into<RecordId> {
    pub fn id(&self) -> RecordId {
        match self {
            Self::Id(id) => id.clone(),
            Self::Record(r) => r.clone().into()
        }
    }
}

impl<T> Into<RecordId> for AdaptiveRelation<T> where T: Clone + Sized + Into<RecordId> {
    fn into(self) -> RecordId {
        self.id().clone()
    }
}

impl<T> From<AdaptiveRelation<T>> for Value where T: Clone + Sized + Into<RecordId> {
    fn from(value: AdaptiveRelation<T>) -> Self {
        Value::Thing(value.id())
    }
}

impl<T> Deref for AdaptiveRelation<T> where T: Clone + Sized + Into<RecordId> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            AdaptiveRelation::Id(_) => panic!("Can not deref AdaptiveRelation, the relation not link to any record (AdaptiveRelation::Record(T))"),
            AdaptiveRelation::Record(r) => {&r}
        }
    }
}
