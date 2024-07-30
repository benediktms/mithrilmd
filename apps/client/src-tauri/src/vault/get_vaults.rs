use entity::entities::vault;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};
use tracing::instrument;

use crate::state::{AppState, RepositoryError};

use super::Vault;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct GetAllVaultsResponse {
    error: Option<RepositoryError>,
    vaults: Vec<Vault>,
}

#[tauri::command]
#[specta::specta]
#[instrument(skip(app), ret)]
pub fn get_all_vaults(app: AppHandle) -> Result<GetAllVaultsResponse, RepositoryError> {
    Ok(
        match tauri::async_runtime::block_on(find_vaults(app.state::<AppState>())) {
            Ok(vaults) => GetAllVaultsResponse {
                error: None,
                vaults,
            },
            Err(e) => GetAllVaultsResponse {
                error: Some(e),
                vaults: vec![],
            },
        },
    )
}

#[instrument(skip(state), ret)]
async fn find_vaults(state: tauri::State<'_, AppState>) -> Result<Vec<Vault>, RepositoryError> {
    let conn = state.conn.lock().await.clone();

    let vaults: Vec<vault::Model> = vault::Entity::find()
        .all(&conn)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

    let vaults = vaults
        .iter()
        .map(|v| Vault {
            id: v.id,
            name: v.name.clone(),
            path: v.path.clone(),
        })
        .collect();

    Ok(vaults)
}
