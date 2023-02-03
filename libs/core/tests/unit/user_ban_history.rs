use std::sync::Arc;
use uuid::Uuid;

use crate::unit::mock_auth_provider::MockApiCognito;
use discuz_core::{
	migration::{Migrator, MigratorTrait},
	service::{auth::constants::UserRole, prelude::*, user::constants::UserStatus},
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};

#[tokio::test]
async fn ban_user_account() {
	let SetupResponse {
		auth_service,
		user_service,
	} = setup().await;

	let admin_user_token = Uuid::new_v4().to_string();
	let user_token = Uuid::new_v4().to_string();
	let ban_reason = "Testing".to_owned();

	let admin = user_service.get_profile(&admin_user_token).await.unwrap();
	let user = user_service.get_profile(&user_token).await.unwrap();

	// Update the user's role
	user_service
		.update_role(&admin.id, &UserRole::Admin)
		.await
		.unwrap();

	let input = BanUserInput {
		ban_user_id: user.id.to_owned(),
		ban_reason: Some(ban_reason.to_owned()),
		ban_time: Some(1000),
		user_id: admin.id.to_owned(),
	};

	let history = user_service.ban_user_account(&input).await.unwrap();

	assert_eq!(history.ban_user_id, user.id);
	assert_eq!(history.ban_reason, Some(ban_reason.to_owned()));
	assert!(history.ban_time.is_some());
	assert!(history.release_time.is_some());

	let user = user_service.get_profile(&user_token).await.unwrap();
	assert_eq!(user.status_id, UserStatus::Banned.to_string());

	assert!(auth_service.validate_user(&user, None).is_err());
}

#[tokio::test]
async fn ban_user_permanently() {
	let SetupResponse {
		auth_service,
		user_service,
	} = setup().await;

	let admin_user_token = Uuid::new_v4().to_string();
	let user_token = Uuid::new_v4().to_string();

	let admin = user_service.get_profile(&admin_user_token).await.unwrap();
	let user = user_service.get_profile(&user_token).await.unwrap();

	// Update the user's role
	user_service
		.update_role(&admin.id, &UserRole::Admin)
		.await
		.unwrap();

	let input = BanUserInput {
		ban_user_id: user.id.to_owned(),
		ban_reason: None,
		ban_time: None,
		user_id: admin.id.to_owned(),
	};

	let history = user_service.ban_user_account(&input).await.unwrap();

	assert_eq!(history.ban_user_id, user.id);
	assert!(history.ban_reason.is_none());
	assert!(history.ban_time.is_none());
	assert!(history.release_time.is_none());

	let user = user_service.get_profile(&user_token).await.unwrap();
	assert_eq!(user.status_id, UserStatus::Banned.to_string());

	assert!(auth_service.validate_user(&user, None).is_err());
}

#[tokio::test]
async fn update_user_ban() {
	let SetupResponse { user_service, .. } = setup().await;

	let admin_user_token = Uuid::new_v4().to_string();
	let user_token = Uuid::new_v4().to_string();
	let ban_reason = "Testing".to_owned();

	let admin = user_service.get_profile(&admin_user_token).await.unwrap();
	let user = user_service.get_profile(&user_token).await.unwrap();

	// Update the user's role
	user_service
		.update_role(&admin.id, &UserRole::Admin)
		.await
		.unwrap();

	let input = BanUserInput {
		ban_user_id: user.id.to_owned(),
		ban_reason: None,
		ban_time: None,
		user_id: admin.id.to_owned(),
	};

	let history = user_service.ban_user_account(&input).await.unwrap();

	let input = UpdateBanHistoryInput {
		id: history.id,
		ban_user_id: user.id.to_owned(),
		ban_reason: Some(ban_reason.to_owned()),
		ban_time: Some(1000),
		release_time: None,
		user_id: admin.id.to_owned(),
	};

	let history = user_service.update_user_ban_history(&input).await.unwrap();

	assert_eq!(history.ban_reason, Some(ban_reason));
	assert_eq!(history.ban_time, Some(1000));
	assert!(history.release_time.is_some());
}

async fn setup() -> SetupResponse {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	let sdk_config = Arc::new(get_aws_sdk_config().await);
	let factory = Factory::new(&db_connection, &sdk_config);
	Migrator::refresh(&db_connection).await.unwrap();
	let api_provider = Arc::new(MockApiCognito);
	let auth_service = Arc::new(AuthService { api_provider });
	let user_service = Arc::new(factory.new_user_service(auth_service.clone()));
	SetupResponse {
		auth_service,
		user_service,
	}
}

pub struct SetupResponse {
	auth_service: Arc<AuthService>,
	user_service: Arc<UserService>,
}
