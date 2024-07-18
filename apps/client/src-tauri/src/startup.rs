use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tauri::AppHandle;

use migration::{Migrator, MigratorTrait};

use crate::util::load_from_env;

pub async fn startup(_app: AppHandle) -> Result<DatabaseConnection, anyhow::Error> {
    let connection_string =
        load_from_env::<String>("DATABASE_URL").expect("Could not read database url");

    let mut opt = ConnectOptions::new(connection_string);
    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let connection: DatabaseConnection = Database::connect(opt).await?;
    Migrator::up(&connection, None).await?;

    Ok(connection)
}
