use crate::entities::user::User;
use crate::web::local_storage::local_storage;

#[derive(Clone, Debug)]
pub struct UserStorage {
    pub data: Option<User>
}

impl UserStorage {
    pub fn new() -> Self {
        let user = local_storage()
            .get("user")
            .expect("Failed to access the local storage")
            .map(|json_value| {
                let current_instance: User = serde_json::from_str(json_value.as_str()).unwrap();
                current_instance
            });

        Self {
            data: user
        }
    }

    pub fn update(&mut self, data: User) {
        local_storage().set_item("user", serde_json::to_string(&data).unwrap().as_str()).expect("Failed to serialize user data into local storage");
        self.data = Some(data);
    }

    pub fn get(&self) -> Option<&User> {
        self.data.as_ref()
    }

    pub fn delete(&mut self) {
        local_storage().remove_item("user").expect("Failed to remove user data from local storage");
        self.data = None;
    }
}
