use rand::Rng;
use crate::services::base::service::{NoParam, Resolve, Service};
use crate::services::user_default_avatar::service::RandomUserDefaultAvatarService;

pub struct RandomUserAvatarServiceApiImpl {}

impl Service<NoParam, String> for RandomUserAvatarServiceApiImpl {
    async fn execute(self, params: NoParam) -> Resolve<String> {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen_range(1..=10);

        Ok(format!("/assets/images/avatars/avatar_{random_number}.png"))
    }
}

impl RandomUserDefaultAvatarService for RandomUserAvatarServiceApiImpl {}
