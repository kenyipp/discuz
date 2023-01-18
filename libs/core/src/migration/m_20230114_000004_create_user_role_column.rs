use crate::utils::db_tools;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
	fn name(&self) -> &str {
		"m_20230114_000004_create_user_role_column"
	}
}

#[async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				sea_query::Table::alter()
					.table(Alias::new("user"))
					.add_column(
						ColumnDef::new(Alias::new("role"))
							.string()
							.default("user")
							.not_null()
							.extra(db_tools::add_column_after(manager, "sub")),
					)
					.to_owned(),
			)
			.await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.alter_table(
				sea_query::Table::alter()
					.table(Alias::new("user"))
					.drop_column(Alias::new("role"))
					.to_owned(),
			)
			.await?;
		Ok(())
	}
}
