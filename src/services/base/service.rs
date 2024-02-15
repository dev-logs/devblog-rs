use std::ops::Deref;
use crate::entities::errors::Errors;

pub type NoParam = ();

pub type VoidResponse = ();

pub type Resolve<T> = Result<T, Errors>;

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
