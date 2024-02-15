use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::Db;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_guess_user::service::{CreateGuestUserService, Params};
use crate::services::user_default_avatar::service::RandomUserDefaultAvatarService;

pub struct CreateGuestUserApiImpl<T> where T: RandomUserDefaultAvatarService {
    pub db: Db,
    pub random_avatar: T
}

impl<T> Service<Params, User> for CreateGuestUserApiImpl<T> where T: RandomUserDefaultAvatarService {
    async fn execute(self, params: Params) -> Resolve<User> {
        let random_avatar_url = self.random_avatar.execute(()).await?;
        let user = User::new(None, Some(random_avatar_url.as_str()), params.display_name.as_str());
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

impl<T> CreateGuestUserService for CreateGuestUserApiImpl<T> where T: RandomUserDefaultAvatarService {}
