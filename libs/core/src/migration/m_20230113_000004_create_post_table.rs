use crate::{constants::MAX_POST_REPLY_COUNT, utils::db_tools::on_update_current_timestamp};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230113_000004_create_post_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.create_table(create_post(manager)).await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(Post::Table).to_owned())
			.await?;
		Ok(())
	}
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
				.default(MAX_POST_REPLY_COUNT)
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
enum User {
	Table,
	UserId,
}
