use sea_orm_migration::prelude::*;

mod m_20221217_000001_create_user_table;

pub struct Migrator;
pub use sea_orm_migration::MigratorTrait;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![Box::new(m_20221217_000001_create_user_table::Migration)]
	}
}
