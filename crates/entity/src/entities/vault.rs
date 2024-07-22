//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use async_trait::async_trait;
use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "vault")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub path: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = chrono::Local::now().to_string();

        Self {
            created_at: Set(now.clone()),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }

    async fn after_save<C>(mut model: Model, _db: &C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert {
            model.created_at = chrono::Local::now().to_string();
        }

        Ok(model)
    }
}
