use surreal_derive_plus::surreal_quote;
use surrealdb_id::link::{Link, NewLink};
use surrealdb_id::relation::{LinkRelation, Relation};
use crate::core_services::surrealdb::Db;
use crate::core_services::surrealdb::user_tbl::UserId;
use crate::entities::blog::Blog;
use crate::entities::errors::Errors;
use crate::entities::like::Like;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::like::perform::service::{LikeBlogService, LikeBlogParam};

pub struct LikeBlogServiceApiImpl {
   pub db: Db
}

impl Service<LikeBlogParam, LinkRelation<User, Like, Blog>> for LikeBlogServiceApiImpl {
    async fn execute(self, params: LikeBlogParam) -> Resolve<LinkRelation<User, Like, Blog>> {
        let user = match params.display_name {
            None => {User::noone()}
            Some(_) => {
                let user_relation: Link<User> = Link::<User>::from(UserId { display_name: params.display_name.as_ref().unwrap().clone() });
                let user_id = user_relation.id();
                let user: Option<User> = self.db.query(surreal_quote!("SELECT * from #val(&user_id)")).await?.take(0)?;
                if user.is_none() {
                    return Err(Errors::NotFound(format!("user name {}", params.display_name.unwrap())));
                }

                user.unwrap()
            }
        };

        let user_relation: Link<User> = Link::<User>::from(UserId { display_name: user.display_name });
        let user_id = user_relation.id();

        let blog_relation: Link<Blog> = Link::<Blog>::new(params.blog_title.clone());
        let blog_id = blog_relation.id();
        let blog: Option<Blog> = self.db.query(surreal_quote!("SELECT * from #val(&blog_id)")).await?.take(0)?;
        if blog.is_none() {
            return Err(Errors::NotFound(format!("blog title {}", params.blog_title)));
        }

        let like = Like { count: params.count };

        let result: Option<LinkRelation<User, Like, Blog>> = self.db.query(surreal_quote!("SELECT * FROM RELATE #val(&user_id) -> like -> #val(&blog_id) #set(&like)")).await?.take(0)?;

        Ok(result.unwrap())
    }
}

impl LikeBlogService for LikeBlogServiceApiImpl {}
