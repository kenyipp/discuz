use std::sync::Arc;
use error_stack::Result;
use discuz_utils::get_db_connection;
use discuz_layers::{
	repository::{ repo_user::RepoUser, database::db_user::DbUser },
	service::{
		auth::{
			errors::AuthError,
			auth_service::{ AuthUser, AuthServiceTrait },
			provider::utils::GetTokensOutput,
		},
		user::user_service::{ UserService, UserServiceTrait, UpdateUserInput },
	},
	migration::{ Migrator, MigratorTrait },
};

#[tokio::test]
async fn user_get_profile() {
	let user_service = setup().await;
	let auth_service = user_service.auth_service.to_owned();
	let auth_user = auth_service.get_auth_user_by_access_token("FAKE_ACCESS_TOKEN").await.unwrap();
	let user = user_service.get_profile("FAKE_SUB").await.unwrap();

	assert_eq!(auth_user.sub, user.sub);
	assert_eq!(auth_user.name, user.name);
	assert_eq!(auth_user.email, user.email);
	assert!(user.avatar_url.is_some());
}

#[tokio::test]
async fn user_update_profile() {
	let user_service = setup().await;
	let user = user_service.get_profile("FAKE_SUB").await.unwrap();

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

async fn setup() -> UserService {
	let db_connection = Arc::new(get_db_connection().await.unwrap());
	Migrator::refresh(&db_connection).await.unwrap();
	let auth_service = Arc::new(MockAuthService);
	let db_user = DbUser::new(&db_connection);
	let repo_user = RepoUser::new(db_user);
	UserService { repo_user, auth_service }
}

#[derive(Debug)]
struct MockAuthService;

#[async_trait]
impl AuthServiceTrait for MockAuthService {
	async fn validate_access_token(&self, _access_token: &str) -> Result<String, AuthError> {
		Ok("FAKE_SUB".to_owned())
	}
	async fn get_auth_user_by_access_token(
		&self,
		_access_token: &str
	) -> Result<AuthUser, AuthError> {
		Ok(AuthUser {
			username: "USERNAME".to_owned(),
			name: "NAME".to_owned(),
			sub: "FAKE_SUB".to_owned(),
			email_verified: Some(false),
			phone_number_verified: Some(false),
			email: "EMAIL".to_owned(),
			picture: Some("https://avataaars.io".to_owned()),
		})
	}
	async fn get_tokens(&self, _code: &str) -> Result<GetTokensOutput, AuthError> {
		Ok(GetTokensOutput {
			id_token: "".to_owned(),
			access_token: "".to_owned(),
			refresh_token: "".to_owned(),
			expires_in: 0,
			token_type: "".to_owned(),
		})
	}
}
