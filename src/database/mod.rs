pub mod entities;

use self::entities::*;
use crate::database::entities::prelude::GuildConfig;
use crate::database::entities::response_images::ActiveModel;
use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use std::time::Duration;

const DEFAULT_DATABASE_URL: &str = "sqlite:./data/sqlite.db";

pub struct DbConn {
    pub client: DatabaseConnection,
}

impl DbConn {
    pub async fn new() -> Self {
        Self {
            client: Database::connect(DEFAULT_DATABASE_URL)
                .await
                .expect("uh oh"),
        }
    }

    pub async fn get_guild_config(&self, guild: i64) -> Result<Option<guild_config::Model>, DbErr> {
        GuildConfig::find_by_id(guild).one(&self.client).await
        // todo generate config if doesn't exist
    }

    pub async fn set_guild_chance(&self, guild: i64, chance: f64) -> Result<(), DbErr> {
        let guild_config = guild_config::ActiveModel {
            id: ActiveValue::Set(guild),
            response_chance: ActiveValue::Set(chance),
        };

        GuildConfig::insert(guild_config)
            .on_conflict(
                OnConflict::columns(vec![guild_config::Column::Id])
                    .update_columns(vec![guild_config::Column::ResponseChance].into_iter())
                    .to_owned(),
            )
            .exec(&self.client)
            .await?;

        Ok(())
    }
}
