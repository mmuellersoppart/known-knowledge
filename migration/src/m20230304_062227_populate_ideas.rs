use std::path::Path;
use std::fs;

use sea_orm_migration::sea_orm::{Set, TransactionTrait, ActiveModelTrait};
use serde_json;

use sea_orm_migration::prelude::*;

use common::transfer_structs::CreateIdea;

#[derive(DeriveMigrationName)]
pub struct Migration;

use entity::idea;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        //transactional approach
        let transaction = db.begin().await?;

        // read in json file
        let bootstrap_ideas_path = Path::new("./migration/test_data/ideas.json");

        // convert to CreateIdea structs
        let json_str = fs::read_to_string(bootstrap_ideas_path).expect("failed to read file {bootstrap_ideas_path:?}");
        let data: Vec<CreateIdea> = serde_json::from_str(&json_str).expect("failed to convert to json");

        // convert those to active models
        let active_ideas: Vec<idea::ActiveModel> = data.into_iter().map(
                |idea_data| {
                    idea::ActiveModel {
                        name: Set(idea_data.name),
                        context: Set(idea_data.context),
                        description: Set(idea_data.description),
                        ..Default::default()
                    }
                }
        ).collect();

        for active_idea in active_ideas {
            active_idea.insert(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .truncate_table(Table::truncate().table(Idea::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Idea {
    Table,
    Id,
    Name,
    Context,
    Description
}
