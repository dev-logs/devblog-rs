use chrono::Utc;
use serde_json::Value;
use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;
use service::Params;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::entities::discussion::Discussion;
use crate::entities::errors::Errors;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service;
use crate::services::create_discussion::service::CreateDiscussionService;
use crate::services::create_guess_user::service::{CreateGuestUserService, Params as CreateGuestUserParams};

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
        let user_relation = AdaptiveRelation::<User>::new(params.display_name.as_str());
        let user_id = user_relation.id();
        let user: Option<User> = self.db.query(surreal_quote!("SELECT * from #val(&user_id)")).await?.take(0)?;

        if user.is_none() {
            return Err(Errors::NotFound(format!("User with display name: {}", params.display_name)));
        }

        let new_discussion = Discussion {
            id: RecordId::from(("discussion", Id::uuid())),
            owner: user_relation,
            content: params.content.to_string(),
            created_at: Utc::now(),
            blog: AdaptiveRelation::<Blog>::new(params.blog_title.as_str()),
        };

        let created_discussion: Option<Discussion> = self.db.query(surreal_quote!(r#"
            RELATE #val(&user_id) -> discuss -> #val(&new_discussion.blog) #set(&new_discussion)
        "#)).await?.take(0)?;

        Ok(created_discussion.unwrap())
    }
}

impl<T> CreateDiscussionService for CreateDiscussionApiImpl<T> where T: CreateGuestUserService {}
