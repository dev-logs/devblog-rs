use chrono::Utc;
use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;
use surrealdb_id::link::Link;
use surrealdb_id::relation::r#trait::IntoRelation;
use surrealdb_id::relation::Relation;
use service::Params;
use crate::core_services::surrealdb::blog_tbl::BlogId;
use crate::core_services::surrealdb::Db;
use crate::core_services::surrealdb::user_tbl::UserId;
use crate::entities::blog::Blog;
use crate::entities::discussion::Discussion;
use crate::entities::errors::Errors;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service;
use crate::services::create_discussion::service::CreateDiscussionService;
use crate::services::create_guess_user::service::CreateGuestUserService;

pub struct CreateDiscussionApiImpl<T> where T: CreateGuestUserService {
    pub db: Db,
    pub create_guess_user: T
}

impl<T> CreateDiscussionApiImpl<T> where T: CreateGuestUserService {
    pub fn new(db: Db, create_guess_user: T) -> Self {
        Self {
            db,
            create_guess_user
        }
    }
}

impl<T> Service<Params, Discussion> for CreateDiscussionApiImpl<T> where T: CreateGuestUserService {
    async fn execute(self, params: Params) -> Resolve<Discussion> {
        let user_relation = Link::<User>::from(UserId {display_name: params.display_name.clone() });
        let user_id = user_relation.id();
        let user: Option<User> = self.db.query(surreal_quote!("SELECT * from #val(&user_id)")).await?.take(0)?;

        if user.is_none() {
            return Err(Errors::UnAuthorization(format!("User with display name: {}", params.display_name)));
        }

        let blog_id = BlogId { title: params.blog_title };
        let new_discussion = Discussion {
            owner: user_relation,
            content: params.content.to_string(),
            created_at: Utc::now(),
            blog: Link::<Blog>::from(blog_id.clone()),
        };

        let relation = new_discussion.relate(user_id, blog_id);
        let created_discussion: Option<Discussion> = self.db.query(surreal_quote!("#relate(&relation)")).await?.take(0)?;

        Ok(created_discussion.unwrap())
    }
}

impl<T> CreateDiscussionService for CreateDiscussionApiImpl<T> where T: CreateGuestUserService {}
