use serde_json::Value;
use crate::db::state::get_client;

pub async fn get_all_users() -> Result<Vec<Value>, String> {
    let client = get_client().await?;
    let admin_db = client.database("admin");
    
    let command = mongodb::bson::doc! { "usersInfo": 1 };
    let result = admin_db.run_command(command, None).await.map_err(|e| e.to_string())?;
    
    let users_bson = result.get_array("users").map_err(|e| e.to_string())?;
    
    let users: Vec<Value> = users_bson
        .iter()
        .filter_map(|u| {
            if let mongodb::bson::Bson::Document(doc) = u {
                mongodb::bson::from_document::<Value>(doc.clone()).ok()
            } else {
                None
            }
        })
        .collect();
    
    Ok(users)
}

pub async fn get_user_by_name(username: String, db: Option<String>) -> Result<Option<Value>, String> {
    let client = get_client().await?;
    let admin_db = client.database("admin");
    
    let user_spec = if let Some(database) = db {
        mongodb::bson::doc! { "user": username, "db": database }
    } else {
        mongodb::bson::doc! { "user": username }
    };
    
    let command = mongodb::bson::doc! { "usersInfo": user_spec };
    let result = admin_db.run_command(command, None).await.map_err(|e| e.to_string())?;
    
    let users_bson = result.get_array("users").map_err(|e| e.to_string())?;
    
    if users_bson.is_empty() {
        return Ok(None);
    }
    
    if let Some(mongodb::bson::Bson::Document(doc)) = users_bson.first() {
        let user: Value = mongodb::bson::from_document(doc.clone()).map_err(|e| e.to_string())?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub async fn create_user(username: String, password: String, db: String, roles: Vec<Value>, custom_data: Option<Value>) -> Result<Value, String> {
    let client = get_client().await?;
    let target_db = client.database(&db);
    
    let roles_bson: Vec<mongodb::bson::Bson> = roles
        .into_iter()
        .filter_map(|r| mongodb::bson::to_bson(&r).ok())
        .collect();
    
    let mut command = mongodb::bson::doc! {
        "createUser": username,
        "pwd": password,
        "roles": roles_bson
    };
    
    if let Some(data) = custom_data {
        if let Ok(bson_data) = mongodb::bson::to_bson(&data) {
            command.insert("customData", bson_data);
        }
    }
    
    let result = target_db.run_command(command, None).await.map_err(|e| e.to_string())?;
    let json_result: Value = mongodb::bson::from_document(result).map_err(|e| e.to_string())?;
    Ok(json_result)
}
