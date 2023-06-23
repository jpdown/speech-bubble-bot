pub mod entities;

use self::entities::*;
use crate::database::entities::prelude::{GuildConfig, ResponseImages, UserOptOut};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use sea_orm::sea_query::OnConflict;
use sea_orm::*;

const DEFAULT_DATA_DIR: &str = "./data";

pub struct DbConn {
    pub client: DatabaseConnection,
}

impl DbConn {
    pub async fn new() -> Self {
        let data_dir = match std::env::var("DATA_DIR") {
            Ok(dir) => dir,
            _ => DEFAULT_DATA_DIR.into(),
        };

        Self {
            client: Database::connect(format!("sqlite:{}/sqlite.db", data_dir))
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

    pub async fn get_all_images(&self, guild: i64) -> Result<Vec<String>, DbErr> {
        let images = ResponseImages::find()
            .filter(response_images::Column::GuildId.eq(guild))
            .all(&self.client)
            .await?;

        let urls = images.into_iter().map(|x| x.url).collect();
        Ok(urls)
    }

    pub async fn get_random_image(&self, guild: i64) -> Result<Option<String>, DbErr> {
        let images = self.get_all_images(guild).await?;

        let chosen = images.choose(&mut thread_rng());
        match chosen {
            Some(img) => Ok(Some(img.clone())),
            _ => Ok(None),
        }
    }

    pub async fn add_image(&self, guild: i64, url: String) -> Result<bool, DbErr> {
        let img = ResponseImages::find()
            .filter(response_images::Column::GuildId.eq(guild))
            .filter(response_images::Column::Url.eq(&url))
            .one(&self.client)
            .await?;

        if img.is_some() {
            return Ok(false);
        }

        let response_image = response_images::ActiveModel {
            guild_id: ActiveValue::Set(guild),
            url: ActiveValue::Set(url),
            ..Default::default()
        };

        ResponseImages::insert(response_image)
            .exec(&self.client)
            .await?;

        Ok(true)
    }

    pub async fn remove_image(&self, guild: i64, url: String) -> Result<u64, DbErr> {
        let res = response_images::Entity::delete_many()
            .filter(response_images::Column::GuildId.eq(guild))
            .filter(response_images::Column::Url.eq(url))
            .exec(&self.client)
            .await?;

        Ok(res.rows_affected)
    }

    pub async fn opt_out(&self, user: i64) -> Result<bool, DbErr> {
        if self.get_opted_out(user).await? {
            return Ok(false);
        }

        let user = user_opt_out::ActiveModel {
            id: ActiveValue::Set(user),
        };
        user_opt_out::Entity::insert(user)
            .on_conflict(
                OnConflict::column(user_opt_out::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&self.client)
            .await?;

        Ok(true)
    }

    pub async fn opt_in(&self, user: i64) -> Result<u64, DbErr> {
        let res = user_opt_out::Entity::delete_by_id(user)
            .exec(&self.client)
            .await?;

        Ok(res.rows_affected)
    }

    pub async fn get_opted_out(&self, user: i64) -> Result<bool, DbErr> {
        let current_opt_out = UserOptOut::find_by_id(user).one(&self.client).await?;

        Ok(current_opt_out.is_some())
    }
}
