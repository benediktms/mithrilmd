use anyhow::Ok;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tauri::{AppHandle, Manager};

use migration::{Migrator, MigratorTrait};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

use crate::{state::AppState, util::load_from_env};

pub async fn init_async(app: AppHandle) -> Result<(), anyhow::Error> {
    let conn = init_db_connection().await?;
    let state = AppState::new(conn);

    app.manage(state);

    Ok(())
}

async fn init_db_connection() -> Result<DatabaseConnection, anyhow::Error> {
    let connection_string =
        load_from_env::<String>("DATABASE_URL").expect("Could not read database url");

    let mut opt = ConnectOptions::new(connection_string);
    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let connection: DatabaseConnection = Database::connect(opt).await?;
    Migrator::up(&connection, None).await?;

    Ok(connection)
}

pub fn init_tracing() {
    let log_fmt = load_from_env::<String>("RUST_LOG").unwrap_or("default".into());
    let log_level = load_from_env::<LevelFilter>("LOG_LEVEL").unwrap_or(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(if log_fmt == "json" {
            fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true)
                .json()
                .boxed()
        } else if log_fmt == "pretty" {
            fmt::layer()
                .with_thread_ids(true)
                .with_thread_names(true)
                .pretty()
                .boxed()
        } else {
            fmt::layer().boxed()
        })
        .with(
            EnvFilter::builder()
                .with_default_directive(log_level.into())
                .from_env_lossy(),
        )
        .init();
}
