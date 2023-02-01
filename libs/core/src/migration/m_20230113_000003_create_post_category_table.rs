use crate::{constants::UNCLASSIFIED_CATEGORY_ID, utils::db_tools::on_update_current_timestamp};
use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230113_000003_create_post_category_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(create_def_post_category(manager))
			.await?;

		if manager.get_database_backend() == DbBackend::MySql {
			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-post_category-parent_id-post_category-post_category_id")
						.from(DefPostCategory::Table, DefPostCategory::ParentId)
						.to(DefPostCategory::Table, DefPostCategory::PostCategoryId)
						.to_owned(),
				)
				.await?;
		}
		manager.exec_stmt(seed_default_category()).await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(DefPostCategory::Table).to_owned())
			.await?;
		Ok(())
	}
}

fn seed_default_category() -> InsertStatement {
	Query::insert()
		.into_table(DefPostCategory::Table)
		.columns([
			DefPostCategory::PostCategoryId,
			DefPostCategory::Name,
			DefPostCategory::Slug,
		])
		.values_panic([
			UNCLASSIFIED_CATEGORY_ID.into(),
			"Chit Chat".into(),
			"chit-chat".into(),
		])
		.to_owned()
}

fn create_def_post_category(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(DefPostCategory::Table)
		.col(
			ColumnDef::new(DefPostCategory::PostCategoryId)
				.string()
				.string_len(64)
				.not_null()
				.primary_key(),
		)
		.col(
			ColumnDef::new(DefPostCategory::Name)
				.string()
				.string_len(50)
				.not_null(),
		)
		.col(
			ColumnDef::new(DefPostCategory::Slug)
				.string()
				.string_len(150)
				.unique_key()
				.not_null(),
		)
		.col(ColumnDef::new(DefPostCategory::Description).text())
		.col(
			ColumnDef::new(DefPostCategory::Level)
				.integer()
				.integer_len(3)
				.unsigned()
				.default(0),
		)
		.col(
			ColumnDef::new(DefPostCategory::Postable)
				.boolean()
				.default(true),
		)
		.col(
			ColumnDef::new(DefPostCategory::ParentId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(ColumnDef::new(DefPostCategory::Count).integer().default(0))
		.col(
			ColumnDef::new(DefPostCategory::SortIndex)
				.integer()
				.default(100),
		)
		.col(
			ColumnDef::new(DefPostCategory::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(DefPostCategory::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(DefPostCategory::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(DefPostCategory::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.foreign_key(
			ForeignKey::create()
				.name("FK-def_post_category-user_id-user-user_id")
				.from(DefPostCategory::Table, DefPostCategory::UserId)
				.to(User::Table, User::UserId),
		)
		.to_owned()
}

#[derive(Iden)]
pub enum DefPostCategory {
	Table,
	PostCategoryId,
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
