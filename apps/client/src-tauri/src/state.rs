use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
}

impl AppState {
    #[must_use]
    pub fn new(conn: DatabaseConnection) -> Self {
        Self {
            conn: Arc::new(conn),
        }
    }
}
