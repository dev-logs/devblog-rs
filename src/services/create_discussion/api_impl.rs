use chrono::Utc;
use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;
use service::Params;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::core_services::surrealdb::Db;
use crate::entities::discussion::Discussion;
use crate::entities::errors::Errors;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service;
use crate::services::create_discussion::service::CreateDiscussionService;

pub struct CreateDiscussionApiImpl {
    pub db: Db
}

impl CreateDiscussionApiImpl {
    pub fn new(db: Db) -> Self {
        Self {
            db
        }
    }
}

impl Service<Params, Discussion> for CreateDiscussionApiImpl {
    async fn execute(self, params: Params) -> Resolve<Discussion> {
        let user: Option<User> = self.db.query(surreal_quote!(r#"SELECT * FROM user:`#&(params.email)`"#))
            .await?.take(0).expect("aaa");
        if user.is_none() {
            return Err(Errors::UnAuthorization);
        }

        let new_discussion = Discussion {
            id: RecordId::from(("discussion", Id::uuid())),
            owner: AdaptiveRelation::<User>::new(user.unwrap().email.clone().as_str()),
            content: params.content.to_string(),
            created_at: Utc::now(),
        };

        let created_discussion: Option<Discussion> = self.db.query(surreal_quote!(r#"
            CREATE #record(&new_discussion);
            SELECT * from #id(&new_discussion) fetch owner
        "#)).await?.take(1)?;

        Ok(created_discussion.unwrap())
    }
}

impl CreateDiscussionService for CreateDiscussionApiImpl {}
