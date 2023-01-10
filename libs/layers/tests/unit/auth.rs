use discuz_layers::{
	migration::{Migrator, MigratorTrait},
	service::{
		auth::{
			auth_service::{AuthService, AuthServiceTrait},
			utils::mock_data::{
				should_return_mock_user_by_access_token, FAKE_ACCESS_TOKEN, FAKE_SUB,
			},
		},
		factory::Factory,
	},
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};
use std::sync::Arc;

#[tokio::test]
async fn get_fake_user() {
	assert!(should_return_mock_user_by_access_token(FAKE_ACCESS_TOKEN));
	let auth_service = setup().await;
	let sub_res = auth_service.validate_access_token(FAKE_ACCESS_TOKEN).await;
	assert!(sub_res.is_ok());
	assert_eq!(sub_res.unwrap(), FAKE_SUB);
	let auth_user_res = auth_service
		.get_auth_user_by_access_token(FAKE_ACCESS_TOKEN)
		.await;
	assert!(auth_user_res.is_ok());
}

#[tokio::test]
async fn should_return_error_by_wrong_access_token() {
	let auth_service = setup().await;
	let sub_res = auth_service
		.validate_access_token("FALSE ACCESS TOKEN")
		.await;
	assert!(sub_res.is_err());
}

async fn setup() -> AuthService {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	Migrator::refresh(&db_connection).await.unwrap();
	let factory = Factory::new(&db_connection, &sdk_config);
	factory.new_auth_service()
}
