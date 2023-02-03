use crate::utils::db_tools::on_update_current_timestamp;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230201_000010_create_config_table"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager.create_table(create_config_table(manager)).await?;
		manager.exec_stmt(seed_config_table()).await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(Config::Table).to_owned())
			.await?;
		Ok(())
	}
}

fn seed_config_table() -> InsertStatement {
	Query::insert()
		.into_table(Config::Table)
		.columns([Config::ConfigKey, Config::ConfigValue])
		.values_panic(["app_status".into(), "normal".into()])
		.values_panic([
			"app_maintaining_message".into(),
			DEFAULT_MAINTENANCE_MESSAGE.into(),
		])
		.to_owned()
}

fn create_config_table(manager: &SchemaManager) -> TableCreateStatement {
	Table::create()
		.table(Config::Table)
		.col(
			ColumnDef::new(Config::ConfigKey)
				.string()
				.string_len(150)
				.not_null(),
		)
		.col(
			ColumnDef::new(Config::ConfigValue)
				.string()
				.text()
				.not_null(),
		)
		.col(
			ColumnDef::new(Config::UserId)
				.string()
				.string_len(64)
				.null(),
		)
		.col(
			ColumnDef::new(Config::StatusId)
				.string()
				.string_len(1)
				.not_null()
				.default("A"),
		)
		.col(
			ColumnDef::new(Config::CreatedAt)
				.timestamp()
				.extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
		)
		.col(
			ColumnDef::new(Config::UpdatedAt)
				.timestamp()
				.extra(on_update_current_timestamp(manager)),
		)
		.foreign_key(
			ForeignKey::create()
				.name("FK-config-user_id-user-user_id")
				.from(Config::Table, Config::UserId)
				.to(User::Table, User::UserId),
		)
		.to_owned()
}

#[derive(Iden)]
pub enum Config {
	Table,
	ConfigKey,
	ConfigValue,
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

const DEFAULT_MAINTENANCE_MESSAGE :&str = "We are currently performing maintenance on our servers to improve their performance and stability.
During this time, our services may be temporarily unavailable.
We apologize for any inconvenience this may cause and appreciate your understanding as we work to improve our system.";
