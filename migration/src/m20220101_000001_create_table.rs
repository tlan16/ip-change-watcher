use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(PublicIps::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(PublicIps::IpV4)
              .text()
              .not_null(),
          )
          .col(
            ColumnDef::new(PublicIps::IpV6)
              .text()
              .not_null(),
          )
          .col(ColumnDef::new(PublicIps::CreatedAt).date_time().not_null().primary_key())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(PublicIps::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum PublicIps {
  Table,
  IpV4,
  IpV6,
  CreatedAt,
}
