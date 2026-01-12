use mongodb::bson::oid::ObjectId;
use serde_json::Value;
use crate::db::state::MongooseState;

pub async fn create_document(state: &MongooseState, collection_name: String, document: Value) -> Result<Value, String> {
    let client = {
        let db_state = state.0.lock().map_err(|_| "Failed to lock mutex".to_string())?;
        db_state.as_ref().ok_or("Database not connected")?.clone()
    };

    let db = client.database("test"); // TODO: Make database name configurable
    let collection = db.collection::<mongodb::bson::Document>(&collection_name);

    let mut bson_doc = mongodb::bson::to_document(&document).map_err(|e| e.to_string())?;
    
    // Ensure _id if not present
    if !bson_doc.contains_key("_id") {
        bson_doc.insert("_id", ObjectId::new());
    }

    collection.insert_one(bson_doc.clone(), None).await.map_err(|e| e.to_string())?;

    let json_doc: Value = mongodb::bson::from_document(bson_doc).map_err(|e| e.to_string())?;
    Ok(json_doc)
}

pub async fn get_document_by_id(state: &MongooseState, collection_name: String, id: String) -> Result<Option<Value>, String> {
    let client = {
        let db_state = state.0.lock().map_err(|_| "Failed to lock mutex".to_string())?;
        db_state.as_ref().ok_or("Database not connected")?.clone()
    };

    let db = client.database("test"); // TODO: Make database name configurable
    let collection = db.collection::<mongodb::bson::Document>(&collection_name);

    let oid = ObjectId::parse_str(&id).map_err(|e| format!("Invalid ID format: {}", e))?;
    let filter = mongodb::bson::doc! { "_id": oid };

    let result = collection.find_one(filter, None).await.map_err(|e| e.to_string())?;

    match result {
        Some(doc) => {
            let json_doc: Value = mongodb::bson::from_document(doc).map_err(|e| e.to_string())?;
            Ok(Some(json_doc))
        },
        None => Ok(None)
    }
}
