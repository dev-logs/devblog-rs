use async_trait::async_trait;
use crate::entities::errors::Errors;

pub type NoParam = ();

pub type VoidResponse = ();

pub type Resolve<T> = Result<T, Errors>;

pub trait Response {}

#[async_trait]
pub trait Service<P, T>
    where
        P: Clone,
{
    async fn execute(self, params: P) -> Resolve<T>;
}
