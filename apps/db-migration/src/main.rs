use sea_orm::*;
use dotenv::dotenv;
use discuz_utils::config::{ Config, Database as DatabaseConfig, get_config };
use discuz_layers::migration::{ Migrator, MigratorTrait };

#[tokio::main]
async fn main() -> Result<(), ()> {
	dotenv().ok();
	// Get the database connection information
	let config = get_config();
	let Config { run_mode, database: database_config, .. } = config;

	let run_mode = run_mode.to_owned();

	assert!(
		run_mode == "production" || run_mode == "ci",
		"Only allow running database migration on the production environment"
	);

	let DatabaseConfig { username, password, port, hostname, database, .. } = database_config;
	let database_url = format!("mysql://{}:{}@{}:{}", username, password, hostname, port);
	let database_name = database.to_owned();

	let db_connection = Database::connect(&database_url).await.expect(
		"Unable to create database connection"
	);

	// Create the database if it isn't created
	db_connection
		.execute(
			Statement::from_string(
				DbBackend::MySql,
				format!("CREATE DATABASE IF NOT EXISTS `{}`;", database_name)
			)
		).await
		.expect("Unable to create database");

	let db_connection = Database::connect(format!("{database_url}/{database_name}")).await.expect(
		"Unable to create database connection to the database"
	);

	// Apply all the database migrations
	Migrator::up(&db_connection, None).await.expect("Unable to migrate the database");
	Ok(())
}
