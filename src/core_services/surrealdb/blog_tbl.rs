use serde_derive::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb_id::link::{Link, NewLink};
use crate::entities::blog::Blog;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlogId {
    pub title: String
}

impl From<BlogId> for RecordId {
    fn from(value: BlogId) -> Self {
        RecordId::from(("blog", value.title.as_str()))
    }
}

impl NewLink<Blog, String> for Link<Blog> {
    fn new(params: String) -> Link<Blog> {
        Link::<Blog>::from(BlogId { title: params })
    }
}
