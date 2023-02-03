use crate::utils::db_tools::on_update_current_timestamp;
use sea_orm::DbBackend;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230124_000007_create_user_ban_history_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(create_user_ban_history_table(manager))
			.await?;

		if manager.get_database_backend() == DbBackend::MySql {
			manager
				.create_index(
					Index::create()
						.table(UserBanHistory::Table)
						.name("IDX-user_ban_history-release_time")
						.index_type(IndexType::BTree)
						.col(UserBanHistory::ReleaseTime)
						.to_owned(),
				)
				.await?;
			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-user_ban_history-ban_user_id-user-user_id")
						.from(UserBanHistory::Table, UserBanHistory::BanUserId)
						.to(User::Table, User::UserId)
						.to_owned(),
				)
				.await?;
			manager
				.create_foreign_key(
					ForeignKey::create()
						.name("FK-user_ban_history-user_id-user-user_id")
						.from(UserBanHistory::Table, UserBanHistory::UserId)
						.to(User::Table, User::UserId)
						.to_owned(),
				)
				.await?;
		}

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(UserBanHistory::Table).to_owned())
			.await?;
		Ok(())
	}
}

fn create_user_ban_history_table(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(UserBanHistory::Table)
		.col(
			ColumnDef::new(UserBanHistory::UserBanHistoryId)
				.integer()
				.integer_len(11)
				.unsigned()
				.not_null()
				.auto_increment()
				.primary_key(),
		)
		.col(
			ColumnDef::new(UserBanHistory::BanUserId)
				.string()
				.string_len(64)
				.not_null(),
		)
		.col(
			ColumnDef::new(UserBanHistory::BanReason)
				.string()
				.string_len(128)
				.null(),
		)
		.col(
			ColumnDef::new(UserBanHistory::BanTime)
				.integer()
				.unsigned()
				.null(),
		)
		.col(
			ColumnDef::new(UserBanHistory::ReleaseTime)
				.timestamp()
				.null(),
		)
		.col(
			ColumnDef::new(UserBanHistory::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(UserBanHistory::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(UserBanHistory::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(UserBanHistory::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.to_owned()
}

#[derive(Iden)]
pub enum UserBanHistory {
	Table,
	UserBanHistoryId,
	BanUserId,
	BanReason,
	BanTime,
	ReleaseTime,
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
