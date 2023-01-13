use sea_orm_migration::prelude::*;

mod m_20221217_000001_create_user_table;
mod m_20230111_000002_create_file_table;

pub struct Migrator;
pub use sea_orm_migration::MigratorTrait;

#[async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m_20221217_000001_create_user_table::Migration),
			Box::new(m_20230111_000002_create_file_table::Migration),
		]
	}
}
