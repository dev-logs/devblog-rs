use lazy_static::lazy_static;
use std::env;

#[derive(Debug)]
pub struct SurrealDb {
    pub socket_address: String,
    pub namespace: String,
    pub db_name: String,
    pub db_password: String,
    pub db_username: String,
}

impl Default for SurrealDb {
    fn default() -> Self {
        SurrealDb {
            socket_address: env::var("DEVLOGS_SURREAL_DB_SOCKET_ADDRESS").unwrap_or("127.0.0.1:8000".to_owned()),
            db_name: env::var("DEVLOGS_SURREAL_DB_NAME").unwrap_or("test".to_owned()),
            namespace: env::var("DEVLOGS_SURREAL_DB_NAMESPACE").unwrap_or("test".to_owned()),
            db_username: env::var("DEVLOGS_SURREAL_DB_USERNAME").unwrap_or("root".to_owned()),
            db_password: env::var("DEVLOGS_SURREAL_DB_PASSWORD").unwrap_or("root".to_owned()),
        }
    }
}

#[derive(Debug, Default)]
pub struct Environment {
    pub surreal_db: SurrealDb,
}

lazy_static! {
    pub static ref ENVIRONMENT: Environment = Default::default();
}
