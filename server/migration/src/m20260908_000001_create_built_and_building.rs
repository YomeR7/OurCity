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
enum Building {
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
enum Construct {
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
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Construct::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Construct::Id)
                            .uuid()
                            .not_null()
                            .default(Expr::cust("gen_random_uuid()"))
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Construct::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Construct::Kind)
                            .enumeration(BuildingKind, BuildingKindVariants::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Construct::X).integer().not_null())
                    .col(ColumnDef::new(Construct::Y).integer().not_null())
                    .col(
                        ColumnDef::new(Construct::Width)
                            .integer()
                            .not_null()
                            .check(Expr::col(Construct::Width).gt(0)),
                    )
                    .col(
                        ColumnDef::new(Construct::Height)
                            .integer()
                            .not_null()
                            .check(Expr::col(Construct::Height).gt(0)),
                    )
                    .col(
                        ColumnDef::new(Construct::Cost)
                            .integer()
                            .not_null()
                            .check(Expr::col(Construct::Cost).gt(0)),
                    )
                    .col(ColumnDef::new(Construct::Votes).integer().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Construct::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(Building::Table).to_owned()).await?;

        manager.drop_type(Type::drop().name(BuildingKind).to_owned()).await?;

        Ok(())
    }
}
