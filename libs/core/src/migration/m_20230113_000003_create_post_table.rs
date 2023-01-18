use crate::{
	constants::{MAX_POST_COMMENT_COUNT, UNCLASSIFIED_CATEGORY_ID},
	utils::db_tools::on_update_current_timestamp,
};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230113_000003_create_post_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.create_table(create_def_post_tag(manager)).await?;
		manager
			.create_table(create_def_post_category(manager))
			.await?;
		manager.create_table(create_post(manager)).await?;
		manager.create_table(create_post_tag(manager)).await?;
		manager.exec_stmt(seed_default_category()).await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(DefPostTag::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(DefPostCategory::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(Post::Table).to_owned())
			.await?;
		manager
			.drop_table(Table::drop().table(PostTag::Table).to_owned())
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
			"Unclassified".into(),
			"unclassified".into(),
		])
		.to_owned()
}

fn create_post(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(Post::Table)
		.col(
			ColumnDef::new(Post::PostId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.primary_key()
				.auto_increment(),
		)
		.col(
			ColumnDef::new(Post::Title)
				.string()
				.string_len(150)
				.not_null(),
		)
		.col(
			ColumnDef::new(Post::CommentCount)
				.integer()
				.integer_len(7)
				.unsigned()
				.default(0)
				.not_null(),
		)
		.col(
			ColumnDef::new(Post::MaxCommentCount)
				.integer()
				.integer_len(7)
				.unsigned()
				.default(MAX_POST_COMMENT_COUNT)
				.not_null(),
		)
		.col(
			ColumnDef::new(Post::Slug)
				.string()
				.string_len(150)
				.unique_key()
				.not_null(),
		)
		.col(
			ColumnDef::new(Post::PostCategoryId)
				.string()
				.string_len(64)
				.not_null(),
		)
		.col(ColumnDef::new(Post::Content).text())
		.col(ColumnDef::new(Post::UserId).string().string_len(64).null())
		.col(
			ColumnDef::new(Post::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(Post::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(Post::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.foreign_key(
			ForeignKey::create()
				.name("FK-post-user_id-user-user_id")
				.from(Post::Table, Post::UserId)
				.to(User::Table, User::UserId),
		)
		.foreign_key(
			ForeignKey::create()
				.name("FK-post-post_category_id-def_post_category-post_category_id")
				.from(Post::Table, Post::UserId)
				.to(User::Table, User::UserId),
		)
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

fn create_def_post_tag(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(DefPostTag::Table)
		.col(
			ColumnDef::new(DefPostTag::PostTagId)
				.string()
				.string_len(64)
				.not_null()
				.primary_key(),
		)
		.col(
			ColumnDef::new(DefPostTag::Name)
				.string()
				.string_len(50)
				.not_null(),
		)
		.col(
			ColumnDef::new(DefPostTag::Slug)
				.string()
				.string_len(150)
				.unique_key()
				.not_null(),
		)
		.col(ColumnDef::new(DefPostTag::Description).text())
		.col(ColumnDef::new(DefPostTag::Count).integer().default(0))
		.col(
			ColumnDef::new(DefPostTag::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(DefPostTag::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(DefPostTag::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(DefPostTag::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.foreign_key(
			ForeignKey::create()
				.name("FK-post_tag-user_id-user-user_id")
				.from(DefPostTag::Table, DefPostTag::UserId)
				.to(User::Table, User::UserId),
		)
		.to_owned()
}

fn create_post_tag(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(PostTag::Table)
		.col(
			ColumnDef::new(PostTag::PostId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null(),
		)
		.col(
			ColumnDef::new(PostTag::PostTagId)
				.string()
				.string_len(64)
				.not_null(),
		)
		.col(
			ColumnDef::new(PostTag::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(PostTag::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(PostTag::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(PostTag::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.primary_key(Index::create().col(PostTag::PostId).col(PostTag::PostTagId))
		.to_owned()
}

#[derive(Iden)]
pub enum Post {
	Table,
	PostId,
	Title,
	Slug,
	PostCategoryId,
	Content,
	CommentCount,
	MaxCommentCount,
	UserId,
	StatusId,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
pub enum PostTag {
	Table,
	PostId,
	PostTagId,
	UserId,
	StatusId,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
pub enum DefPostCategory {
	Table,
	PostCategoryId,
	Name,
	Slug,
	Description,
	ParentId,
	Count,
	SortIndex,
	UserId,
	StatusId,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
pub enum DefPostTag {
	Table,
	PostTagId,
	Name,
	Slug,
	Description,
	Count,
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
