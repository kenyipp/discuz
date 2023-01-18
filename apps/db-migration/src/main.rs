use db_migration::utils::database::get_db_connection;
use discuz_core::migration::{Migrator, MigratorTrait};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), ()> {
	dotenv().ok();
	let db_connection = get_db_connection().await;
	// Apply all the database migrations
	Migrator::up(&db_connection, None)
		.await
		.expect("Unable to migrate the database");
	Ok(())
}
