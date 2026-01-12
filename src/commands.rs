use tauri::{command, State, Runtime};
use crate::db::{MongooseState, connect_to_db, create_document, get_document_by_id};

#[command]
pub async fn connect(state: State<'_, MongooseState>, url: String) -> Result<(), String> {
    connect_to_db(&state, url).await
}

#[command]
pub async fn create(state: State<'_, MongooseState>, collection: String, document: serde_json::Value) -> Result<serde_json::Value, String> {
    create_document(&state, collection, document).await
}

#[command]
pub async fn get_by_id(state: State<'_, MongooseState>, collection: String, id: String) -> Result<Option<serde_json::Value>, String> {
    get_document_by_id(&state, collection, id).await
}
