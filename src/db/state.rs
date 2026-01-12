use std::sync::OnceLock;
use mongodb::Client;
use tokio::sync::RwLock;

static CONNECTION: OnceLock<RwLock<Option<Client>>> = OnceLock::new();
static DB_NAME: OnceLock<RwLock<String>> = OnceLock::new();

fn get_connection() -> &'static RwLock<Option<Client>> {
    CONNECTION.get_or_init(|| RwLock::new(None))
}

fn get_db_name_store() -> &'static RwLock<String> {
    DB_NAME.get_or_init(|| RwLock::new("test".to_string()))
}

pub async fn set_client(client: Client) {
    let mut conn = get_connection().write().await;
    *conn = Some(client);
}

pub async fn get_client() -> Result<Client, String> {
    let conn = get_connection().read().await;
    conn.clone().ok_or_else(|| "Database not connected".to_string())
}

pub async fn is_connected() -> bool {
    let conn = get_connection().read().await;
    conn.is_some()
}

pub async fn set_db_name(name: String) {
    let mut db = get_db_name_store().write().await;
    *db = name;
}

pub async fn get_db_name() -> String {
    let db = get_db_name_store().read().await;
    db.clone()
}
