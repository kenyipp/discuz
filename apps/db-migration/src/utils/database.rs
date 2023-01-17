use discuz_utils::config::{get_config, Config, Database as DatabaseConfig};
use sea_orm::*;

pub async fn get_db_connection() -> DatabaseConnection {
	// Get the database connection information
	let config = get_config();
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
	let database_url = format!("mysql://{}:{}@{}:{}", username, password, hostname, port);
	let database_name = database.to_owned();

	let db_connection = Database::connect(&database_url)
		.await
		.expect("Unable to create database connection");

	// Create the database if it isn't created
	db_connection
		.execute(Statement::from_string(
			DbBackend::MySql,
			format!("CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;", database_name),
		))
		.await
		.expect("Unable to create database");

	Database::connect(format!("{database_url}/{database_name}"))
		.await
		.expect("Unable to create database connection to the database")
}
