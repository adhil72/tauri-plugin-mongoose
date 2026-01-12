use tauri::command;
use serde_json::Value;
use crate::db::{connect_to_db, create_document, get_document_by_id};

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
