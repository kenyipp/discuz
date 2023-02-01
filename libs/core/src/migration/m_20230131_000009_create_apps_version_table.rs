use crate::utils::db_tools::{create_unique_key, on_update_current_timestamp};
use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230131_000009_create_apps_version"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(create_apps_version_table(manager))
			.await?;

		manager
			.get_connection()
			.execute(create_unique_key(
				&AppsVersion::Table.to_string(),
				"UK_platform_package_id",
				vec![
					AppsVersion::Platform.to_string(),
					AppsVersion::PackageId.to_string(),
				],
				manager.get_database_backend(),
			))
			.await
			.unwrap();

		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(AppsVersion::Table).to_owned())
			.await?;
		Ok(())
	}
}

fn create_apps_version_table(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(AppsVersion::Table)
		.col(
			ColumnDef::new(AppsVersion::AppsVersionId)
				.string()
				.string_len(64)
				.not_null()
				.primary_key(),
		)
		.col(
			ColumnDef::new(AppsVersion::Platform)
				.string()
				.string_len(10)
				.null(),
		)
		.col(
			ColumnDef::new(AppsVersion::PackageId)
				.string()
				.string_len(20)
				.null(),
		)
		.col(
			ColumnDef::new(AppsVersion::CurrentVersion)
				.string()
				.string_len(10)
				.null(),
		)
		.col(
			ColumnDef::new(AppsVersion::MinimalVersion)
				.string()
				.string_len(10)
				.null(),
		)
		.col(
			ColumnDef::new(AppsVersion::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(AppsVersion::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(AppsVersion::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.foreign_key(
			ForeignKey::create()
				.name("FK-apps_version-user_id-user-user_id")
				.from(AppsVersion::Table, AppsVersion::UserId)
				.to(User::Table, User::UserId),
		)
		.to_owned()
}

#[derive(Iden)]
pub enum AppsVersion {
	Table,
	AppsVersionId,
	Platform,
	PackageId,
	CurrentVersion,
	MinimalVersion,
	UserId,
	CreatedAt,
	UpdatedAt,
}

#[derive(Iden)]
enum User {
	Table,
	UserId,
}
