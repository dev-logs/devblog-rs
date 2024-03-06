use crate::api::web_controllers::user::create_guest_user::create_guest_user;
use crate::entities::user::User;
use crate::services::base::service::{Resolve, Service};
use crate::services::create_guess_user::service::{CreateGuestUserService, Params};
use crate::web::local_storage::user::UserStorage;

pub struct CreateGuestUserWebImpl {}

impl Service<Params, User> for CreateGuestUserWebImpl {
    async fn execute(self, params: Params) -> Resolve<User> {
        let created_guest = create_guest_user(params).await?;
        let mut user_storage = UserStorage::new();
        user_storage.update(created_guest.clone());

        Ok(created_guest)
    }
}

impl CreateGuestUserService for CreateGuestUserWebImpl {}