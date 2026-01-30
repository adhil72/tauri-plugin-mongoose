use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub mod db;
mod commands;

pub use db::{connect_to_db, get_client, is_connected, create_document, get_document_by_id, set_db_name, get_db_name, get_all_users, get_user_by_name, create_user};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("mongoose")
        .invoke_handler(tauri::generate_handler![commands::connect, commands::create, commands::get_by_id, commands::get_users, commands::get_user, commands::create_db_user])
        .build()
}
