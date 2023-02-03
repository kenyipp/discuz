use std::sync::Arc;

use discuz_core::{
	migration::{Migrator, MigratorTrait},
	service::prelude::*,
	utils::mock_data::apps_version::create_mock_apps_version,
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};

#[tokio::test]
async fn get_app_config() {
	let SetupResponse { config_service } = setup().await;
	let result = config_service.get_config().await;
	assert!(result.is_ok());
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	let factory = Factory::new(&db_connection, &sdk_config);
	let config_service = Arc::new(factory.new_config_service());
	Migrator::refresh(&db_connection).await.unwrap();
	create_mock_apps_version(&db_connection).await;
	SetupResponse { config_service }
}

pub struct SetupResponse {
	config_service: Arc<ConfigService>,
}
