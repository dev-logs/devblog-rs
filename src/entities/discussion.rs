use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surreal_derive_plus::SurrealDerive;
use surrealdb::opt::RecordId;
use surrealdb::sql::{Id, Thing};
use crate::core_services::surrealdb::adaptive_relation::AdaptiveRelation;
use crate::entities::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(SurrealDerive))]
pub struct Discussion {
    pub id: RecordId,
    pub owner: AdaptiveRelation<User>,
    pub content: String,
    pub created_at: DateTime<Utc>
}

#[cfg(feature = "ssr")]
impl Into<Thing> for Discussion {
    fn into(self) -> Thing {
        self.id.clone()
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    mod user_wrapper {
        use super::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct User1 {
            pub email: String,
            pub age: i32,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UserType2 {
            pub email: String,
            pub id: i32,
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum UserWrapper {
            User1(User1),
            UserType2(UserType2),
        }
    }

    #[test]
    fn test() {
        let json1 = r#"{"email": "user@example.com", "age": 30}"#;
        let user1: user_wrapper::UserWrapper = serde_json::from_str(json1).unwrap();
        println!("Deserialized User1: {:?}", user1);

        let json2 = r#"{"email": "user@example.com", "id": 42}"#;
        let user2: user_wrapper::UserWrapper = serde_json::from_str(json2).unwrap();
        println!("Deserialized UserType2: {:?}", user2);

        let user1_json = serde_json::to_string(&user1).unwrap();
        println!("Serialized User1: {}", user1_json);

        let user2_json = serde_json::to_string(&user2).unwrap();
        println!("Serialized UserType2: {}", user2_json);
    }
}