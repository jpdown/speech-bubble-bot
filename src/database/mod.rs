pub mod entities;

use self::entities::*;
use crate::database::entities::prelude::{GuildConfig, ResponseImages};
use crate::database::entities::response_images::ActiveModel;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use sea_orm::sea_query::{Expr, Func, OnConflict, Query, SimpleExpr};
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

    pub async fn get_image(&self, guild: i64) -> Result<Option<String>, DbErr> {
        let images = ResponseImages::find()
            .filter(response_images::Column::GuildId.eq(guild))
            .all(&self.client)
            .await?;

        let chosen = images.choose(&mut thread_rng());

        match chosen {
            Some(img) => Ok(Some(img.url.clone())),
            _ => Ok(None),
        }
    }

    pub async fn add_image(&self, guild: i64, url: String) -> Result<(), DbErr> {
        let response_image = response_images::ActiveModel {
            guild_id: ActiveValue::Set(guild),
            url: ActiveValue::Set(url.clone()),
            ..Default::default()
        };

        let img = ResponseImages::find()
            .filter(response_images::Column::GuildId.eq(guild))
            .filter(response_images::Column::Url.eq(url))
            .one(&self.client)
            .await?;

        if img.is_some() {
            return Ok(());
        }

        ResponseImages::insert(response_image)
            .exec(&self.client)
            .await?;

        Ok(())
    }

    pub async fn remove_image(&self, guild: i64, url: String) -> Result<(), DbErr> {
        let res = response_images::Entity::delete_many()
            .filter(response_images::Column::GuildId.eq(guild))
            .filter(response_images::Column::Url.eq(url))
            .exec(&self.client)
            .await?;

        Ok(())
    }
}
