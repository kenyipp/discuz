use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::SchemaManager;

pub fn on_update_current_timestamp(manager: &SchemaManager) -> String {
	match manager.get_database_backend() {
		DatabaseBackend::MySql => {
			"DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned()
		}
		DatabaseBackend::Sqlite => "DEFAULT CURRENT_TIMESTAMP".to_owned(),
		_ => {
			panic!("on_update_current_timestamp function is work on MySql and Sqlite backend only")
		}
	}
}

pub fn add_column_after(manager: &SchemaManager, column: &str) -> String {
	match manager.get_database_backend() {
		DatabaseBackend::MySql => format!("AFTER {column}"),
		_ => "".to_owned(),
	}
}

pub fn create_unique_key(
	table_name: &str,
	index_name: &str,
	column_list: Vec<String>,
	backend: DatabaseBackend,
) -> Statement {
	let string = match backend {
		DatabaseBackend::Sqlite | DatabaseBackend::MySql => format!(
			"CREATE UNIQUE INDEX {index_name} ON {table_name} ({})",
			column_list.join(",")
		),
		_ => panic!("create_unique_key function is work on Sqlite and MySql backend only"),
	};
	Statement::from_string(backend, string)
}
