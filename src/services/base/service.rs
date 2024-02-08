use crate::entities::discussion::Discussion;
use crate::entities::errors::Errors;
use crate::entities::user::User;

pub type NoParam = ();

pub type VoidResponse = ();

pub type Resolve<T> = Result<T, Errors>;

pub trait Response {}

pub trait Service<P, T>
    where
        P: Clone,
{
    async fn execute(self, params: P) -> Resolve<T>;
}
