use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use std::sync::Mutex;

mod db;
mod commands;

use db::MongooseState;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("mongoose")
        .invoke_handler(tauri::generate_handler![commands::connect, commands::create, commands::get_by_id])
        .setup(|app, _api| {
            app.manage(MongooseState(Mutex::new(None)));
            Ok(())
        })
        .build()
}
