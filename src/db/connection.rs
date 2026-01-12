use mongodb::{Client, options::ClientOptions};
use crate::db::state::{set_client, set_db_name};

pub async fn connect_to_db(url: String, db_name: Option<String>) -> Result<(), String> {
    let client_options = ClientOptions::parse(&url).await.map_err(|e| e.to_string())?;
    let client = Client::with_options(client_options).map_err(|e| e.to_string())?;
    
    set_client(client).await;
    
    if let Some(name) = db_name {
        set_db_name(name).await;
    }
    
    Ok(())
}
