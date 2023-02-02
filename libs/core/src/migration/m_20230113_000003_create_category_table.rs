use crate::{constants::UNCLASSIFIED_CATEGORY_ID, utils::db_tools::on_update_current_timestamp};
use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230113_000003_create_category_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.create_table(create_category(manager)).await?;

		if manager.get_database_backend() == DbBackend::MySql {
			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-category-parent_id-category-category_id")
						.from(Category::Table, Category::ParentId)
						.to(Category::Table, Category::CategoryId)
						.to_owned(),
				)
				.await?;
		}
		manager.exec_stmt(seed_default_category()).await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(Category::Table).to_owned())
			.await?;
		Ok(())
	}
}

fn seed_default_category() -> InsertStatement {
	Query::insert()
		.into_table(Category::Table)
		.columns([Category::CategoryId, Category::Name, Category::Slug])
		.values_panic([
			UNCLASSIFIED_CATEGORY_ID.into(),
			"Chit Chat".into(),
			"chit-chat".into(),
		])
		.to_owned()
}

fn create_category(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(Category::Table)
		.col(
			ColumnDef::new(Category::CategoryId)
				.string()
				.string_len(64)
				.not_null()
				.primary_key(),
		)
		.col(
			ColumnDef::new(Category::Name)
				.string()
				.string_len(50)
				.not_null(),
		)
		.col(
			ColumnDef::new(Category::Slug)
				.string()
				.string_len(150)
				.unique_key()
				.not_null(),
		)
		.col(ColumnDef::new(Category::Description).text())
		.col(
			ColumnDef::new(Category::Level)
				.integer()
				.integer_len(3)
				.unsigned()
				.default(0),
		)
		.col(ColumnDef::new(Category::Postable).boolean().default(true))
		.col(
			ColumnDef::new(Category::ParentId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(ColumnDef::new(Category::Count).integer().default(0))
		.col(ColumnDef::new(Category::SortIndex).integer().default(100))
		.col(
			ColumnDef::new(Category::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(Category::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(Category::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(Category::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.foreign_key(
			ForeignKey::create()
				.name("FK-category-user_id-user-user_id")
				.from(Category::Table, Category::UserId)
				.to(User::Table, User::UserId),
		)
		.to_owned()
}

#[derive(Iden)]
pub enum Category {
	Table,
	CategoryId,
	Name,
	Slug,
	Description,
	Level,
	Postable,
	ParentId,
	Count,
	SortIndex,
	UserId,
	StatusId,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
enum User {
	Table,
	UserId,
}
