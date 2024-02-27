use std::ops::Deref;
use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;

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
