use discuz_core::{
	constants::{FAKE_ACCESS_TOKEN, FAKE_SUB},
	migration::{Migrator, MigratorTrait},
	repository::{database::db_user::DbUser, repo_user::RepoUser},
	service::{
		auth::{
			auth_service::{AuthService, AuthServiceTrait},
			constants::UserRole,
			utils::mock_data::should_return_mock_user_by_access_token,
		},
		factory::Factory,
		user::user_service::{UserService, UserServiceTrait},
	},
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};
use std::sync::Arc;

#[tokio::test]
async fn get_fake_user() {
	assert!(should_return_mock_user_by_access_token(FAKE_ACCESS_TOKEN));
	let SetupResponse { auth_service, .. } = setup().await;
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
	let SetupResponse { auth_service, .. } = setup().await;
	let sub_res = auth_service
		.validate_access_token("FALSE ACCESS TOKEN")
		.await;
	assert!(sub_res.is_err());
}

#[tokio::test]
async fn should_reject_if_user_is_not_permitted() {
	let SetupResponse {
		auth_service,
		user_service,
		..
	} = setup().await;

	let user = user_service.get_profile(FAKE_ACCESS_TOKEN).await.unwrap();
	assert_eq!(user.role, UserRole::User.to_string());
	let is_permitted = auth_service.validate_user(&user, Some(&[UserRole::Admin]));
	assert!(is_permitted.is_err());
}

#[tokio::test]
async fn should_accept_if_user_is_permitted() {
	let SetupResponse {
		auth_service,
		user_service,
		..
	} = setup().await;

	let user = user_service.get_profile(FAKE_ACCESS_TOKEN).await.unwrap();
	assert_eq!(user.role, UserRole::User.to_string());
	let is_permitted = auth_service.validate_user(&user, Some(&[UserRole::Admin, UserRole::User]));

	assert!(is_permitted.is_ok());
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	Migrator::refresh(&db_connection).await.unwrap();
	let db_user = DbUser::new(&db_connection);
	let repo_user = RepoUser::new(db_user);
	let factory = Factory::new(&db_connection, &sdk_config);
	let auth_service = Arc::new(factory.new_auth_service());
	let user_service = Arc::new(UserService {
		repo_user,
		auth_service: auth_service.clone(),
	});
	SetupResponse {
		auth_service,
		user_service,
	}
}

pub struct SetupResponse {
	auth_service: Arc<AuthService>,
	user_service: Arc<UserService>,
}
