use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::Db;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_guess_user::service::{CreateGuestUserService, Params};

pub struct CreateGuestUserApiImpl {
    pub(crate) db: Db
}

impl Service<Params, User> for CreateGuestUserApiImpl {
    async fn execute(self, params: Params) -> Resolve<User> {
        let user = User::new(None, None, params.display_name.as_str());
        let found_existing_user: Option<User> = self.db.query(surreal_quote!(r#"SELECT * FROM #id(&user)"#)).await?.take(0)?;
        match found_existing_user {
            None => {
                let created_user: Option<User> = self.db
                    .query(surreal_quote!(r#"CREATE #record(&user)"#))
                    .await?
                    .take(0)?;
                Ok(created_user.unwrap())
            }
            Some(user) => {Ok(user)}
        }
    }
}

impl CreateGuestUserService for CreateGuestUserApiImpl {}
