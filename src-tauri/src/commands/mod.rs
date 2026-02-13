pub mod auth;
pub mod graph;
pub mod mashes;
pub mod pipeline;
pub mod search;
pub mod settings;

use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub struct DbState(pub Arc<Mutex<Connection>>);
