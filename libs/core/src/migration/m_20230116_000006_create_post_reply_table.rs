use crate::utils::db_tools::on_update_current_timestamp;
use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230116_000006_create_post_reply_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(create_post_reply_table(manager))
			.await?;

		// Since the Sqlite does not support modification of foreign key constraints to existing tables
		if manager.get_database_backend() == DbBackend::MySql {
			// We need to create the index separately since SQLite can't create the index when creating the table
			manager
				.create_index(
					Index::create()
						.table(PostReply::Table)
						.name("IDX-post_reply-status_id")
						.index_type(IndexType::BTree)
						.col(PostReply::StatusId)
						.to_owned(),
				)
				.await?;

			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-post_reply-post_id-post-post_id")
						.from(PostReply::Table, PostReply::PostId)
						.to(Post::Table, Post::PostId)
						.to_owned(),
				)
				.await?;

			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-post_reply-user_id-user-user_id")
						.from(PostReply::Table, PostReply::UserId)
						.to(User::Table, User::UserId)
						.to_owned(),
				)
				.await?;

			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-post_reply-quote_reply_id-post_reply-post_reply_id")
						.from(PostReply::Table, PostReply::QuoteReplyId)
						.to(PostReply::Table, PostReply::PostReplyId)
						.to_owned(),
				)
				.await?;
		}
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(PostReply::Table).to_owned())
			.await?;
		Ok(())
	}
}

fn create_post_reply_table(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(PostReply::Table)
		.col(
			ColumnDef::new(PostReply::PostReplyId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.primary_key(),
		)
		.col(
			ColumnDef::new(PostReply::QuoteReplyId)
				.integer()
				.integer_len(11)
				.unsigned(),
		)
		.col(
			ColumnDef::new(PostReply::NoOfReply)
				.integer()
				.integer_len(11)
				.unsigned(),
		)
		.col(
			ColumnDef::new(PostReply::PostId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null(),
		)
		.col(ColumnDef::new(PostReply::Content).text().not_null())
		.col(
			ColumnDef::new(PostReply::LikeCount)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.default(0),
		)
		.col(
			ColumnDef::new(PostReply::DislikeCount)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.default(0),
		)
		.col(
			ColumnDef::new(PostReply::LowQuality)
				.boolean()
				.default(false),
		)
		.col(
			ColumnDef::new(PostReply::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(PostReply::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(PostReply::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(PostReply::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.to_owned()
}

#[derive(Iden)]
pub enum PostReply {
	Table,
	PostReplyId,
	NoOfReply,
	QuoteReplyId,
	Content,
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
