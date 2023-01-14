use discuz_layers::migration::{Migrator, MigratorTrait};
use discuz_utils::get_db_connection;
use std::sync::Arc;

#[tokio::test]
async fn db_migration_test() {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	Migrator::refresh(&db_connection).await.unwrap();
}
