use std::ops::Deref;
use std::vec::IntoIter;
use serde_derive::{Deserialize, Serialize};
use crate::entities::errors::Errors;

pub type NoParam = ();

pub type VoidResponse = ();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PagingParam {
    pub page: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub data: Vec<T>,
    pub page: i32,
    pub total_page: i32,
    pub total_record: i32
}

impl<T> IntoIterator for PageResponse<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

pub type Resolve<T> = Result<T, Errors>;

pub fn error_message<T>(resolve: Option<Resolve<T>>) -> Option<String> {
    if (resolve.is_none()) {
        return None;
    }

    resolve.unwrap().err().map(|it| it.to_string())
}

#[derive(Clone, Debug)]
pub struct ClonedResult<T, E> where T: Clone, E: Clone {
    result: Result<T, E>
}

impl<T, E> From<Result<T, E>> for ClonedResult<T, E> where T: Clone, E: Clone {
    fn from(value: Result<T, E>) -> Self {
        Self {
            result: value
        }
    }
}

impl<T, E> Deref for ClonedResult<T, E> where T: Clone, E: Clone {
    type Target = Result<T, E>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

pub trait Response {}

pub trait Service<P, T>
    where
        P: Clone,
{
    async fn execute(self, params: P) -> Resolve<T>;
}
