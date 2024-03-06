use surreal_derive_plus::surreal_quote;
use crate::core_services::surrealdb::Db;
use crate::entities::errors::Errors;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service, VoidResponse};
use crate::services::subscribe::service::{Params, SubscribeService};

pub struct SubscribeServiceApiImpl {
    pub db: Db
}

impl Service<Params, VoidResponse> for SubscribeServiceApiImpl {
    async fn execute(self, params: Params) -> Resolve<VoidResponse> {
        let user: Option<User> = self.db.query(surreal_quote!("SELECT * FROM user WHERE email=#val(&params.email)")).await?.take(0)?;
        if user.is_some() {
            return Err(Errors::AlreadyExist("User already subscribe".to_string()));
        }

        let new_user = User::new(Some(params.email.as_str()), None, params.email.as_str());
        let user: Option<User> = self.db.query(surreal_quote!("CREATE #record(&new_user)")).await?.take(0)?;

        Ok(())
    }
}

impl SubscribeService for SubscribeServiceApiImpl {}
