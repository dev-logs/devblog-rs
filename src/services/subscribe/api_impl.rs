use surreal_derive_plus::surreal_quote;
use surrealdb::opt::RecordId;
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
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
        let user: Option<User> = match params.display_name {
            None => {
                self.db.query(surreal_quote!("SELECT * FROM user WHERE email=#val(&params.email)")).await?.take(0)?
            }
            Some(it) => {
                let user_id: RecordId = AdaptiveRelation::<User>::new(it.as_str()).id().clone();
                let user: Option<User> = self.db.query(surreal_quote!("SELECT * FROM #val(&user_id)")).await?.take(0)?;
                user
            }
        };

        if user.is_some() {
            if user.as_ref().unwrap().email.as_ref().eq(&Some(&params.email)) {
                return Err(Errors::AlreadyExist("This email already exist".to_string()));
            }

            let updatedUser: Option<User> = self.db.query(surreal_quote!("UPDATE #id(user.as_ref().unwrap()) SET email=#val(&params.email)")).await?.take(0)?;
            return Ok(());
        }

        let new_user = User::new(Some(params.email.as_str()), None, params.email.as_str());
        let user: Option<User> = self.db.query(surreal_quote!("CREATE #record(&new_user)")).await?.take(0)?;

        Ok(())
    }
}

impl SubscribeService for SubscribeServiceApiImpl {}
