use entity::entities::vault;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};
use tracing::instrument;

use crate::state::{AppState, RepositoryError};

use super::Vault;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct FindVaultResponse {
    vault: Option<Vault>,
    error: Option<RepositoryError>,
}

#[tauri::command]
#[specta::specta]
#[instrument(skip(app), ret)]
pub fn find_vault(app: AppHandle, id: i32) -> Result<FindVaultResponse, RepositoryError> {
    Ok(
        match tauri::async_runtime::block_on(find_vault_by_id(app.state::<AppState>(), id)) {
            Ok(vault) => FindVaultResponse {
                error: None,
                vault: Some(vault),
            },
            Err(e) => FindVaultResponse {
                error: Some(e),
                vault: None,
            },
        },
    )
}

#[instrument(skip(state), ret)]
async fn find_vault_by_id(
    state: tauri::State<'_, AppState>,
    id: i32,
) -> Result<Vault, RepositoryError> {
    let conn = state.conn.lock().await.clone();

    let found = vault::Entity::find_by_id(id)
        .one(&conn)
        .await
        .map_err(|_| RepositoryError::NotFoundByPrimaryKey("vault".into(), id))?;

    if let Some(vault) = found {
        return Ok(Vault {
            id: vault.id,
            name: vault.name,
            path: vault.path,
        });
    }

    Err(RepositoryError::DatabaseError("foo".into()))
}
