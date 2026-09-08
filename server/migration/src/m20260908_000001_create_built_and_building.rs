use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
struct BuildingKind;

#[derive(DeriveIden, EnumIter)]
enum BuildingKindVariants {
    #[sea_orm(iden = "House")]
    House,
}

#[derive(DeriveIden)]
enum Built {
    Table,
    Id,
    CreatedAt,
    Kind,
    X,
    Y,
    Width,
    Height,
}

#[derive(DeriveIden)]
enum Building {
    Table,
    Id,
    CreatedAt,
    Kind,
    X,
    Y,
    Width,
    Height,
    Cost,
    Votes,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(BuildingKind)
                    .values(BuildingKindVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Built::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Built::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Built::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Built::Kind)
                            .enumeration(BuildingKind, BuildingKindVariants::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Built::X).integer().not_null())
                    .col(ColumnDef::new(Built::Y).integer().not_null())
                    .col(
                        ColumnDef::new(Built::Width)
                            .integer()
                            .not_null()
                            .check(Expr::col(Built::Width).gt(0)),
                    )
                    .col(
                        ColumnDef::new(Built::Height)
                            .integer()
                            .not_null()
                            .check(Expr::col(Built::Height).gt(0)),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Building::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Building::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Building::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Building::Kind)
                            .enumeration(BuildingKind, BuildingKindVariants::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Building::X).integer().not_null())
                    .col(ColumnDef::new(Building::Y).integer().not_null())
                    .col(
                        ColumnDef::new(Building::Width)
                            .integer()
                            .not_null()
                            .check(Expr::col(Building::Width).gt(0)),
                    )
                    .col(
                        ColumnDef::new(Building::Height)
                            .integer()
                            .not_null()
                            .check(Expr::col(Building::Height).gt(0)),
                    )
                    .col(
                        ColumnDef::new(Building::Cost)
                            .integer()
                            .not_null()
                            .check(Expr::col(Building::Cost).gt(0)),
                    )
                    .col(
                        ColumnDef::new(Building::Votes)
                            .integer()
                            .not_null()
                            .check(Expr::col(Building::Votes).gt(0)),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Building::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Built::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(BuildingKind).to_owned())
            .await?;

        Ok(())
    }
}
