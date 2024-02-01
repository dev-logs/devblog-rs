use chrono::{DateTime, Utc};
use crate::entities::author::Author;

pub struct Blog {
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub url: String,
    pub author: Author
}

impl Blog {
    pub fn new (
        url: &'static str,
        title: &'static str,
        description: &'static str,
        author: Author,
        created_at: DateTime<Utc>
    ) -> Self {

        Self {
            url: url.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            created_at,
            author,
        }
    }
}
