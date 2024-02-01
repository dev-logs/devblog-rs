/*
* Handle logic of initiating author
* all author must be listed here because it is local development only,
* so don't need to be confused why the fn name is a name of someone.
*/
use crate::entities::author::Author;

pub trait AuthorProviderService {
    fn tiendang(&self) -> Author;

    fn list(&self) -> Vec<Author>;
}
