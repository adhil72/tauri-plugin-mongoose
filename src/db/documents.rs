use mongodb::{bson::{oid::ObjectId, Document}, options::{FindOptions as MongoFindOptions, FindOneOptions as MongoFindOneOptions}};
use serde_json::Value;
use serde::Deserialize;
use futures::stream::TryStreamExt;
use crate::db::state::{get_client, get_db_name};

#[derive(Debug, Deserialize)]
pub struct SearchOptions {
    pub skip: Option<u64>,
    pub limit: Option<i64>,
    pub page: Option<u64>,
    pub sort: Option<Value>,
}

pub async fn create_document(collection_name: String, document: Value) -> Result<Value, String> {
    let client = get_client().await?;
    let db_name = get_db_name().await;
    
    let db = client.database(&db_name);
    let collection = db.collection::<Document>(&collection_name);

    let mut bson_doc = mongodb::bson::to_document(&document).map_err(|e| e.to_string())?;
    
    if !bson_doc.contains_key("_id") {
        bson_doc.insert("_id", ObjectId::new());
    }

    collection.insert_one(bson_doc.clone(), None).await.map_err(|e| e.to_string())?;

    let json_doc: Value = mongodb::bson::from_document(bson_doc).map_err(|e| e.to_string())?;
    Ok(json_doc)
}

pub async fn get_document_by_id(collection_name: String, id: String) -> Result<Option<Value>, String> {
    let client = get_client().await?;
    let db_name = get_db_name().await;
    
    let db = client.database(&db_name);
    let collection = db.collection::<Document>(&collection_name);

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

pub async fn find_documents(collection_name: String, filter: Option<Value>, options: Option<SearchOptions>) -> Result<Vec<Value>, String> {
    let client = get_client().await?;
    let db_name = get_db_name().await;
    
    let db = client.database(&db_name);
    let collection = db.collection::<Document>(&collection_name);

    let mut find_options = MongoFindOptions::builder().build();
    
    if let Some(opts) = options {
        if let Some(limit) = opts.limit {
            find_options.limit = Some(limit);
        }
        
        let mut skip = opts.skip.unwrap_or(0);
        if let Some(page) = opts.page {
           if page > 0 {
                let limit = opts.limit.unwrap_or(10); // Default limit 10 if page is set
                skip = (page as u64 - 1) * (limit as u64);
                // Also ensure limit is set if it wasn't
                if find_options.limit.is_none() {
                    find_options.limit = Some(limit);
                }
           }
        }
        find_options.skip = Some(skip);

        if let Some(sort) = opts.sort {
            if let Ok(sort_doc) = mongodb::bson::to_document(&sort) {
                 find_options.sort = Some(sort_doc);
            }
        }
    }

    let query = if let Some(f) = filter {
        mongodb::bson::to_document(&f).map_err(|e| e.to_string())?
    } else {
        mongodb::bson::doc! {}
    };

    let mut cursor = collection.find(query, find_options).await.map_err(|e| e.to_string())?;
    
    let mut docs = Vec::new();
    while let Some(result) = cursor.try_next().await.map_err(|e| e.to_string())? {
        let json_doc: Value = mongodb::bson::from_document(result).map_err(|e| e.to_string())?;
        docs.push(json_doc);
    }
    
    Ok(docs)
}

pub async fn find_one_document(collection_name: String, filter: Option<Value>, options: Option<SearchOptions>) -> Result<Option<Value>, String> {
    let client = get_client().await?;
    let db_name = get_db_name().await;
    
    let db = client.database(&db_name);
    let collection = db.collection::<Document>(&collection_name);

    let mut find_options = MongoFindOneOptions::builder().build();

    if let Some(opts) = options {
       if let Some(skip) = opts.skip {
           find_options.skip = Some(skip);
       }
       
       if let Some(sort) = opts.sort {
            if let Ok(sort_doc) = mongodb::bson::to_document(&sort) {
                 find_options.sort = Some(sort_doc);
            }
       }
       // Note: limit and page doesn't make much sense for findOne, usually. 
       // Start/skip/sort apply.
    }

    let query = if let Some(f) = filter {
        mongodb::bson::to_document(&f).map_err(|e| e.to_string())?
    } else {
        mongodb::bson::doc! {}
    };

    let result = collection.find_one(query, find_options).await.map_err(|e| e.to_string())?;

    match result {
        Some(doc) => {
            let json_doc: Value = mongodb::bson::from_document(doc).map_err(|e| e.to_string())?;
            Ok(Some(json_doc))
        },
        None => Ok(None)
    }
}
