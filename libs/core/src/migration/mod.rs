use sea_orm_migration::prelude::*;

mod m_20221217_000001_create_user_table;
mod m_20230111_000002_create_file_table;
mod m_20230113_000003_create_post_category_table;
mod m_20230113_000004_create_post_table;
mod m_20230114_000005_create_user_role_column;
mod m_20230116_000006_create_post_reply_table;
mod m_20230124_000007_create_user_ban_history_table;
mod m_20230130_000008_create_default_categories;
mod m_20230131_000009_create_apps_version_table;

pub struct Migrator;
pub use sea_orm_migration::MigratorTrait;

#[async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m_20221217_000001_create_user_table::Migration),
			Box::new(m_20230111_000002_create_file_table::Migration),
			Box::new(m_20230113_000003_create_post_category_table::Migration),
			Box::new(m_20230113_000004_create_post_table::Migration),
			Box::new(m_20230114_000005_create_user_role_column::Migration),
			Box::new(m_20230116_000006_create_post_reply_table::Migration),
			Box::new(m_20230124_000007_create_user_ban_history_table::Migration),
			Box::new(m_20230130_000008_create_default_categories::Migration),
			Box::new(m_20230131_000009_create_apps_version_table::Migration),
		]
	}
}
