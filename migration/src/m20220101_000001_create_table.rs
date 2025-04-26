use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Entry::Table)
					.if_not_exists()
					.col(pk_auto(Entry::Id))
					.col(string_null(Entry::AppName))
					.col(string_null(Entry::WindowTitle))
					.col(text(Entry::ExtractedText))
					.col(timestamp(Entry::CreatedAt).not_null())
					.col(timestamp(Entry::UpdatedAt).not_null())
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.drop_table(Table::drop().table(Entry::Table).to_owned()).await
	}
}

#[derive(DeriveIden)]
enum Entry {
	Table,
	Id,
	AppName,
	WindowTitle,
	ExtractedText,
	CreatedAt,
	UpdatedAt,
}
