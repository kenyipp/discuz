use sea_orm::DatabaseBackend;
use sea_orm_migration::SchemaManager;

pub fn on_update_current_timestamp(manager: &SchemaManager) -> String {
	match manager.get_database_backend() {
		DatabaseBackend::MySql => {
			"DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned()
		}
		DatabaseBackend::Sqlite => "DEFAULT CURRENT_TIMESTAMP".to_owned(),
		_ => todo!("Database migration isn't implemented for current driver"),
	}
}

pub fn add_column_after(manager: &SchemaManager, column: &str) -> String {
	match manager.get_database_backend() {
		DatabaseBackend::MySql => format!("AFTER {}", column),
		_ => "".to_owned(),
	}
}
