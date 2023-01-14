use crate::utils::db::on_update_current_timestamp;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230111_000002_create_file_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(File::Table)
					.col(
						ColumnDef::new(File::FileId)
							.string()
							.string_len(64)
							.not_null()
							.primary_key(),
					)
					.col(
						ColumnDef::new(File::Name)
							.string()
							.string_len(150)
							.not_null(),
					)
					.col(ColumnDef::new(File::AlternativeText).text().null())
					.col(ColumnDef::new(File::Caption).text().null())
					.col(ColumnDef::new(File::Description).text().null())
					.col(ColumnDef::new(File::UserId).string().string_len(64).null())
					.col(
						ColumnDef::new(File::MimeType)
							.string()
							.string_len(64)
							.null(),
					)
					.col(
						ColumnDef::new(File::Size)
							.decimal()
							.decimal_len(8, 2)
							.null(),
					)
					.col(ColumnDef::new(File::PublicUri).text().null())
					.col(
						ColumnDef::new(File::StatusId)
							.string()
							.string_len(1)
							.not_null()
							.default("A"),
					)
					.col(
						ColumnDef::new(File::CreatedAt)
							.timestamp()
							.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
					)
					.col(
						ColumnDef::new(File::UpdatedAt)
							.timestamp()
							.extra(on_update_current_timestamp(manager)),
					)
					.foreign_key(
						ForeignKey::create()
							.name("FK-file-user_id-user-user_id")
							.from(File::Table, File::UserId)
							.to(User::Table, User::UserId),
					)
					.to_owned(),
			)
			.await?;

		Ok(())
	}
	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(File::Table).to_owned())
			.await?;
		Ok(())
	}
}

#[derive(Iden)]
enum User {
	Table,
	UserId,
}

#[derive(Iden)]
pub enum File {
	Table,
	FileId,
	Name,
	AlternativeText,
	Caption,
	Description,
	MimeType,
	Size,
	PublicUri,
	UserId,
	StatusId,
	CreatedAt,
	UpdatedAt,
}
