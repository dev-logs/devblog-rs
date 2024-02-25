use leptos::*;
use crate::entities::user::User;
use crate::services::create_guess_user::service::Params;

#[server(CreateGuestUser, "/web")]
pub async fn create_guest_user(params: Params) -> Result<User, ServerFnError> {
    use crate::services::base::service::*;
    use crate::core_services::api_di::*;

    let create_guest = ApiInjector::service_injector().get_create_guest_user_service();

    let created_guest = create_guest.execute(params).await?;
    Ok(created_guest)
}
