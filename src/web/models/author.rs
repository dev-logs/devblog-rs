use crate::entities::author::Author;

#[derive(Debug)]
pub struct AuthorWebModel {
    pub name: String,
    pub avatar_url: String,
}

impl AuthorWebModel {
    fn new(name: &str, avatar_url: &str) -> Self {
        Self {
            name: name.to_string(),
            avatar_url: avatar_url.to_string(),
        }
    }
}

impl From<Author> for AuthorWebModel {
    fn from(value: Author) -> Self {
        Self {
            name: value.name.clone(),
            avatar_url: value.avatar_url.clone(),
        }
    }
}
