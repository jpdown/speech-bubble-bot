use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GuildConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GuildConfig::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GuildConfig::ResponseChance)
                            .float()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(ResponseImages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ResponseImages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ResponseImages::GuildId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ResponseImages::Url).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(GuildConfig::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ResponseImages::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum GuildConfig {
    Table,
    Id,
    ResponseChance,
}

#[derive(Iden)]
enum ResponseImages {
    Table,
    Id,
    GuildId,
    Url,
}
