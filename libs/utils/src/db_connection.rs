use error_stack::{IntoReport, Result, ResultExt};
use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::config::{get_config, Config, Database as DatabaseConfig};

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
	let config = get_config().to_owned();
	let url = get_connection_string(&config);
	Database::connect(url)
		.await
		.into_report()
		.change_context(DbErr::Custom("Unable to connect the database".to_string()))
}

fn get_connection_string(config: &Config) -> String {
	if let Some(db_url) = config.database.url.to_owned() {
		db_url
	} else {
		let Config {
			database: database_config,
			..
		} = config;
		let DatabaseConfig {
			username,
			password,
			port,
			hostname,
			database,
			..
		} = database_config;
		format!("mysql://{username}:{password}@{hostname}:{port}/{database}")
	}
}
