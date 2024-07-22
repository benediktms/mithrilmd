use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};
use tracing::instrument;

use crate::state::{AppState, ApplicationError};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SetupVaultInput {
    name: String,
    location: String,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct SetupVaultResponse {
    success: bool,
    error: Option<ApplicationError>,
}

#[tauri::command]
#[specta::specta]
#[instrument(skip(app), ret)]
pub fn setup_new_vault(
    app: AppHandle,
    input: SetupVaultInput,
) -> Result<SetupVaultResponse, ApplicationError> {
    let SetupVaultInput { name, location } = input;
    println!("starting setup for new vault vault {name} at location {location}");

    Ok(
        match tauri::async_runtime::block_on(create_vault(app.state::<AppState>(), name, location))
        {
            Ok(()) => SetupVaultResponse {
                success: true,
                error: None,
            },
            Err(e) => SetupVaultResponse {
                success: false,
                error: Some(e),
            },
        },
    )
}

async fn create_vault(
    state: tauri::State<'_, AppState>,
    name: String,
    location: String,
) -> Result<(), ApplicationError> {
    let conn = state.conn.lock().await.clone();
    conn.ping()
        .await
        .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;

    println!("creating vault {name} at location {location}");

    Ok(())
}
