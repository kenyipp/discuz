pub use super::utils::AuthUser;
use crate::service::auth::{
	errors::AuthError,
	provider::{api_provider::ApiCognitoTrait, utils::GetTokensOutput},
	utils::{
		get_auth_user_by_access_token,
		mock_data::{get_fake_sub, get_mock_auth_user, should_return_mock_user_by_access_token},
	},
};
use error_stack::{Result, ResultExt};
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct AuthService {
	pub api_provider: Arc<dyn ApiCognitoTrait>,
}

impl AuthService {
	pub fn new(api_provider: &Arc<dyn ApiCognitoTrait>) -> AuthService {
		AuthService {
			api_provider: api_provider.clone(),
		}
	}
}

#[async_trait]
pub trait AuthServiceTrait: Sync + Send + Debug {
	async fn validate_access_token(&self, access_token: &str) -> Result<String, AuthError>;
	async fn get_auth_user_by_access_token(
		&self,
		access_token: &str,
	) -> Result<AuthUser, AuthError>;
	async fn get_tokens(&self, code: &str) -> Result<GetTokensOutput, AuthError>;
}

#[async_trait]
impl AuthServiceTrait for AuthService {
	async fn validate_access_token(&self, access_token: &str) -> Result<String, AuthError> {
		if should_return_mock_user_by_access_token(access_token) {
			return Ok(get_fake_sub());
		}
		self.api_provider
			.validate_access_token(access_token)
			.await
			.change_context(AuthError::InvalidAccessTokenError)
	}
	async fn get_auth_user_by_access_token(
		&self,
		access_token: &str,
	) -> Result<AuthUser, AuthError> {
		if should_return_mock_user_by_access_token(access_token) {
			return Ok(get_mock_auth_user());
		}
		get_auth_user_by_access_token(&*self.api_provider, access_token).await
	}
	async fn get_tokens(&self, code: &str) -> Result<GetTokensOutput, AuthError> {
		self.api_provider
			.get_tokens(code)
			.await
			.change_context(AuthError::Generic("Invalid auth code".to_string()))
	}
}
