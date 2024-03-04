use chrono::Utc;
use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::core_services::surrealdb::Db;
use crate::entities::blog::Blog;
use crate::entities::errors::Errors;
use crate::entities::relation::relation::Relation;
use crate::entities::user::User;
use crate::entities::view::View;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::blog_detail::read::mark_read_service::{MarkReadService, Params};

pub struct MarkReadServiceApiImpl {
    pub db: Db,
}

impl Service<Params, VoidResponse> for MarkReadServiceApiImpl {
    async fn execute(self, params: Params) -> Resolve<VoidResponse> {
        let user = match params.user {
            None => {User::noone()}
            Some(user) => {
                let found_user: Option<User> = self.db.query(surreal_quote!("SELECT * FROM #id(&user)")).await?.take(0)?;
                if found_user.is_none() {
                    return Err(Errors::NotFound(format!("User not found, display_name={}", user.display_name)))
                }

                found_user.unwrap()
            }
        };

        let blog_relation = AdaptiveRelation::<Blog>::new(params.blog_title.as_str());
        let blog_id = blog_relation.id();
        let view = View { view_at: Utc::now() };
        let _: Option<Relation<View, User, Blog>> = self.db.query(surreal_quote!("SELECT * FROM RELATE #id(&user) -> view -> #val(&blog_id) #set(&view)")).await?.take(0)?;

        Ok(())
    }
}

impl MarkReadService for MarkReadServiceApiImpl {}