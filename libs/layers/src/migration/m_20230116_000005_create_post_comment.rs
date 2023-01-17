use crate::utils::db_tools::on_update_current_timestamp;
use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230116_000005_create_post_comment"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(create_post_comment_table(manager))
			.await?;

		// Since the Sqlite does not support modification of foreign key constraints to existing tables
		if manager.get_database_backend() == DbBackend::MySql {
			// We need to create the index separately since SQLite can't create the index when creating the table
			manager
				.create_index(
					Index::create()
						.table(PostComment::Table)
						.name("IDX-post_comment-status_id")
						.index_type(IndexType::BTree)
						.col(PostComment::StatusId)
						.to_owned(),
				)
				.await?;

			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-post_comment-post_id-post-post_id")
						.from(PostComment::Table, PostComment::PostId)
						.to(Post::Table, Post::PostId)
						.to_owned(),
				)
				.await?;

			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-post_comment-user_id-user-user_id")
						.from(PostComment::Table, PostComment::UserId)
						.to(User::Table, User::UserId)
						.to_owned(),
				)
				.await?;

			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-post_comment-quote_comment_id-post_comment-post_comment_id")
						.from(PostComment::Table, PostComment::QuoteCommentId)
						.to(PostComment::Table, PostComment::PostCommentId)
						.to_owned(),
				)
				.await?;
		}
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(PostComment::Table).to_owned())
			.await?;
		Ok(())
	}
}

fn create_post_comment_table(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(PostComment::Table)
		.col(
			ColumnDef::new(PostComment::PostCommentId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.primary_key(),
		)
		.col(
			ColumnDef::new(PostComment::QuoteCommentId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null(),
		)
		.col(
			ColumnDef::new(PostComment::PostId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null(),
		)
		.col(
			ColumnDef::new(PostComment::LikeCount)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.default(0),
		)
		.col(
			ColumnDef::new(PostComment::DislikeCount)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.default(0),
		)
		.col(
			ColumnDef::new(PostComment::LowQuality)
				.boolean()
				.default(false),
		)
		.col(
			ColumnDef::new(PostComment::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(PostComment::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(PostComment::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(PostComment::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.to_owned()
}

#[derive(Iden)]
pub enum PostComment {
	Table,
	PostCommentId,
	QuoteCommentId,
	PostId,
	LikeCount,
	DislikeCount,
	LowQuality,
	UserId,
	StatusId,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
enum Post {
	Table,
	PostId,
}

#[derive(Iden)]
enum User {
	Table,
	UserId,
}
