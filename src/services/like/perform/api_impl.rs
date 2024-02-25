use std::fmt::{Debug, format};
use leptos::html::form;
use leptos::tracing::instrument::WithSubscriber;
use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::entities::errors::Errors;
use crate::entities::like::Like;
use crate::entities::relation::r#trait::IntoRelation;
use crate::entities::relation::relation::Relation;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::like::perform::service::{LikeBlogService, LikeBlogParam};

pub struct LikeBlogServiceApiImpl {
   pub db: Db
}

impl Service<LikeBlogParam, Relation<Like, User, Blog>> for LikeBlogServiceApiImpl {
    async fn execute(self, params: LikeBlogParam) -> Resolve<Relation<Like, User, Blog>> {
        let user = match params.display_name {
            None => {User::noone()}
            Some(_) => {
                let user_relation: AdaptiveRelation<User> = AdaptiveRelation::<User>::new(params.display_name.as_ref().unwrap());
                let user_id = user_relation.id();
                let user: Option<User> = self.db.query(surreal_quote!("SELECT * from #val(&user_id)")).await?.take(0)?;
                if user.is_none() {
                    return Err(Errors::NotFound(format!("user name {}", params.display_name.unwrap())));
                }

                user.unwrap()
            }
        };

        let user_relation: AdaptiveRelation<User> = AdaptiveRelation::<User>::new(user.display_name.as_str());
        let user_id = user_relation.id();

        let blog_relation:  AdaptiveRelation<Blog> = AdaptiveRelation::<Blog>::new(params.blog_title.as_str());
        let blog_id = blog_relation.id();
        let blog: Option<Blog> = self.db.query(surreal_quote!("SELECT * from #val(&blog_id)")).await?.take(0)?;
        if blog.is_none() {
            return Err(Errors::NotFound(format!("blog title {}", params.blog_title)));
        }

        let like = Like { count: params.count };
        let relation = like.into_relation(user_relation, blog_relation);

        let result: Option<Relation<Like, User, Blog>> = self.db.query(surreal_quote!("SELECT * FROM RELATE #val(&user_id) -> like -> #val(&blog_id) #set(&like)")).await?.take(0)?;

        Ok(result.unwrap())
    }
}

impl LikeBlogService for LikeBlogServiceApiImpl {}
