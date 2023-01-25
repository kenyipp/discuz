use sea_orm_migration::prelude::*;

mod m_20221217_000001_create_user_table;
mod m_20230111_000002_create_file_table;
mod m_20230113_000003_create_post_table;
mod m_20230114_000004_create_user_role_column;
mod m_20230116_000005_create_post_reply_table;
mod m_20230124_000006_create_user_ban_history_table;

pub struct Migrator;
pub use sea_orm_migration::MigratorTrait;

#[async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m_20221217_000001_create_user_table::Migration),
			Box::new(m_20230111_000002_create_file_table::Migration),
			Box::new(m_20230113_000003_create_post_table::Migration),
			Box::new(m_20230114_000004_create_user_role_column::Migration),
			Box::new(m_20230116_000005_create_post_reply_table::Migration),
			Box::new(m_20230124_000006_create_user_ban_history_table::Migration),
		]
	}
}
