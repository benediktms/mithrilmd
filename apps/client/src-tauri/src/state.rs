use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::sync::Arc;
use tauri::async_runtime::Mutex;

#[derive(Default, Debug)]
pub struct AppState {
    pub conn: Arc<Mutex<DatabaseConnection>>,
}

impl AppState {
    #[must_use]
    pub fn new(conn: DatabaseConnection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize, Type)]
pub enum ApplicationError {
    #[error("Query failed: {0}")]
    DatabaseError(String),
}
