mod find_vault;
mod get_vaults;
mod setup_vault;

pub use find_vault::*;
pub use get_vaults::*;
pub use setup_vault::*;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub(crate) struct Vault {
    id: i32,
    name: String,
    path: String,
}
