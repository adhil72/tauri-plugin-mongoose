pub mod state;
pub mod connection;
pub mod documents;

pub use state::{get_client, is_connected, set_db_name, get_db_name};
pub use connection::connect_to_db;
pub use documents::{create_document, get_document_by_id};
