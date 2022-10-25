use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .add_column(ColumnDef::new(Book::Author).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .drop_column(Book::Author)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum Book {
    Table,
    Author,
}
