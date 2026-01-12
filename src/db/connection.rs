use mongodb::{Client, options::ClientOptions};
use crate::db::state::MongooseState;

pub async fn connect_to_db(state: &MongooseState, url: String) -> Result<(), String> {
    let mut client_options = ClientOptions::parse(&url).await.map_err(|e| e.to_string())?;
    let client = Client::with_options(client_options).map_err(|e| e.to_string())?;

    let mut db_state = state.0.lock().map_err(|_| "Failed to lock mutex".to_string())?;
    *db_state = Some(client);

    Ok(())
}
