use discuz_core::{
	constants::FAKE_ACCESS_TOKEN,
	migration::{Migrator, MigratorTrait},
	repository::{database::db_user::DbUser, repo_user::RepoUser},
	service::{
		auth::auth_service::{AuthService, AuthServiceTrait},
		factory::Factory,
		user::user_service::{UpdateUserInput, UserService, UserServiceTrait},
	},
};
use discuz_utils::{amazon::get_aws_sdk_config, get_db_connection};
use dotenv::dotenv;
use std::sync::Arc;

#[tokio::test]
async fn user_get_profile() {
	let SetupResponse {
		auth_service,
		user_service,
		..
	} = setup().await;
	let auth_user = auth_service
		.get_auth_user_by_access_token(FAKE_ACCESS_TOKEN)
		.await
		.unwrap();
	let user = user_service.get_profile(FAKE_ACCESS_TOKEN).await.unwrap();

	assert_eq!(auth_user.sub, user.sub);
	assert_eq!(auth_user.name, user.name);
	assert_eq!(auth_user.email, user.email);
	assert!(user.avatar_url.is_none());
}

#[tokio::test]
async fn user_update_profile() {
	let SetupResponse { user_service, .. } = setup().await;
	let user = user_service.get_profile(FAKE_ACCESS_TOKEN).await.unwrap();

	let update = UpdateUserInput {
		id: user.id.to_owned(),
		name: "ANOTHER_NAME".to_owned(),
		avatar_url: None,
	};
	user_service.update(&update).await.unwrap();
	let updated_user = user_service.find_by_id(&user.id).await.unwrap().unwrap();

	assert_eq!(updated_user.name, "ANOTHER_NAME".to_owned());
	assert!(updated_user.avatar_url.is_none());
}

async fn setup() -> SetupResponse {
	dotenv().ok();
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
		user_service,
		auth_service,
	}
}

pub struct SetupResponse {
	user_service: Arc<UserService>,
	auth_service: Arc<AuthService>,
}
