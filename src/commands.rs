use tauri::command;
use serde_json::Value;
use crate::db::{connect_to_db, create_document, get_document_by_id, get_all_users, get_user_by_name, create_user, find_documents, find_one_document, SearchOptions};

#[command(rename_all = "camelCase")]
pub async fn connect(url: String, db_name: Option<String>) -> Result<(), String> {
    connect_to_db(url, db_name).await
}

#[command]
pub async fn create(collection: String, document: Value) -> Result<Value, String> {
    create_document(collection, document).await
}

#[command]
pub async fn get_by_id(collection: String, id: String) -> Result<Option<Value>, String> {
    get_document_by_id(collection, id).await
}

#[command]
pub async fn get_users() -> Result<Vec<Value>, String> {
    get_all_users().await
}

#[command]
pub async fn get_user(username: String, db: Option<String>) -> Result<Option<Value>, String> {
    get_user_by_name(username, db).await
}

#[command(rename_all = "camelCase")]
pub async fn create_db_user(username: String, password: String, db: String, roles: Vec<Value>, custom_data: Option<Value>) -> Result<Value, String> {
    create_user(username, password, db, roles, custom_data).await
}

#[command]
pub async fn find(collection: String, filter: Option<Value>, options: Option<SearchOptions>) -> Result<Vec<Value>, String> {
    find_documents(collection, filter, options).await
}

#[command(rename_all = "camelCase")]
pub async fn find_one(collection: String, filter: Option<Value>, options: Option<SearchOptions>) -> Result<Option<Value>, String> {
    find_one_document(collection, filter, options).await
}
