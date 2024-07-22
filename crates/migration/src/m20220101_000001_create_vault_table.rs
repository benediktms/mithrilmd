use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(Vault::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vault::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Vault::Path).string().not_null())
                    .col(ColumnDef::new(Vault::Name).string().not_null())
                    .col(
                        ColumnDef::new(Vault::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Vault::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await;

        // this gets done in the active model implementation of the entity
        // let db = manager.get_connection();
        // db.execute_unprepared(
        //     "CREATE TRIGGER vault_updated_at
        //         AFTER
        //           UPDATE
        //         ON vault
        //         FOR EACH ROW
        //     BEGIN
        //         UPDATE vault
        //         SET updated_at = CURRENT_TIMESTAMP
        //         WHERE id = OLD.id;
        //     END",
        // )
        // .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vault::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Vault {
    Table,
    Id,
    Path,
    Name,
    CreatedAt,
    UpdatedAt,
}
