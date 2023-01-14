use crate::utils::db::on_update_current_timestamp;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20221217-000001_create_user_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		// Create the user table
		manager
			.create_table(
				Table::create()
					.table(User::Table)
					.col(
						ColumnDef::new(User::UserId)
							.string()
							.string_len(64)
							.not_null()
							.primary_key(),
					)
					.col(ColumnDef::new(User::Sub).string().string_len(64).not_null())
					.col(
						ColumnDef::new(User::Name)
							.string()
							.string_len(150)
							.not_null(),
					)
					.col(
						ColumnDef::new(User::Email)
							.string()
							.string_len(250)
							.not_null()
							.unique_key(),
					)
					.col(ColumnDef::new(User::Phone).string().string_len(150))
					.col(ColumnDef::new(User::AvatarUrl).text())
					.col(ColumnDef::new(User::Notes).text())
					.col(
						ColumnDef::new(User::StatusId)
							.string()
							.string_len(1)
							.not_null()
							.default("A"),
					)
					.col(
						ColumnDef::new(User::CreatedAt)
							.timestamp()
							.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
					)
					.col(
						ColumnDef::new(User::UpdatedAt)
							.timestamp()
							.extra(on_update_current_timestamp(manager)),
					)
					.to_owned(),
			)
			.await?;
		Ok(())
	}
	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(User::Table).to_owned())
			.await?;
		Ok(())
	}
}

#[derive(Iden)]
pub enum User {
	Table,
	Sub,
	UserId,
	Name,
	Email,
	Phone,
	Notes,
	AvatarUrl,
	StatusId,
	CreatedAt,
	UpdatedAt,
}
