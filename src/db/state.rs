use std::sync::Mutex;
use mongodb::Client;

pub struct MongooseState(pub Mutex<Option<Client>>);
