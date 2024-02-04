use chrono::Utc;
use surreal_derive_plus::surreal_quote;
use service::{Params, Response};
use crate::core_services::surrealdb::Db;
use crate::entities::discussion::Discussion;
use crate::entities::errors::Errors;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_discussion::service;
use crate::services::create_discussion::service::CreateDiscussionService;

pub struct CreateDiscussionApiImpl {
    db: Db
}

impl CreateDiscussionApiImpl {
    pub fn new(db: Db) -> Self {
        Self {
            db
        }
    }
}

#[async_trait::async_trait]
impl Service<Params, Response> for CreateDiscussionApiImpl {
    async fn execute(self, params: Params) -> Resolve<Response> {
        let user: Option<User> = self.db.query(surreal_quote!(r#"SELECT #id(&params.user)"#))
            .await?.take(0)?;
        if user.is_none() {
            return Err(Errors::UnAuthorization);
        }

        let new_discussion = Discussion {
            owner: user.unwrap().clone(),
            content: params.content.to_string(),
            created_at: Utc::now(),
        };

        let created_discussion = self.db.query(surreal_quote!(r#"
            CREATE #record(&new_discussion)
        "#));
        Ok(Response {})
    }
}

impl CreateDiscussionService for CreateDiscussionApiImpl {}
