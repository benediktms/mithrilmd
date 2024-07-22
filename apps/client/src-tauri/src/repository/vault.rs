use entity::entities::vault;
use sea_orm::{EntityTrait, Set};
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

    Ok(
        match tauri::async_runtime::block_on(create_vault(app.state::<AppState>(), name, location))
        {
            Ok(_) => SetupVaultResponse {
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

// TODO: add tests for this
#[instrument(skip(state), ret)]
async fn create_vault(
    state: tauri::State<'_, AppState>,
    name: String,
    location: String,
) -> Result<i32, ApplicationError> {
    let conn = state.conn.lock().await.clone();
    conn.ping()
        .await
        .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;

    let new_vault = vault::ActiveModel {
        path: Set(location),
        name: Set(name),
        ..Default::default()
    };

    let res = vault::Entity::insert(new_vault)
        .exec(&conn)
        .await
        .map_err(|e| ApplicationError::DatabaseError(e.to_string()))?;

    Ok(res.last_insert_id)
}
