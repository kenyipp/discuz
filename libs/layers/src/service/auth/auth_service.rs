use std::{ sync::Arc, fmt::Debug };
use super::utils::get_auth_user_by_access_token;
use super::errors::AuthError;
use super::provider::{ utils::GetTokensOutput, api_provider::ApiCognitoTrait };
pub use super::utils::AuthUser;
use error_stack::{ Result, ResultExt };

#[derive(Debug, Clone)]
pub struct AuthService {
	pub api_provider: Arc<dyn ApiCognitoTrait>,
}

impl AuthService {
	pub fn new(api_provider: &Arc<dyn ApiCognitoTrait>) -> AuthService {
		AuthService { api_provider: api_provider.clone() }
	}
}

#[async_trait]
pub trait AuthServiceTrait: Sync + Send + Debug {
	async fn validate_access_token(&self, access_token: &str) -> Result<String, AuthError>;
	async fn get_auth_user_by_access_token(
		&self,
		access_token: &str
	) -> Result<AuthUser, AuthError>;
	async fn get_tokens(&self, code: &str) -> Result<GetTokensOutput, AuthError>;
}

#[async_trait]
impl AuthServiceTrait for AuthService {
	async fn validate_access_token(&self, access_token: &str) -> Result<String, AuthError> {
		self.api_provider
			.validate_access_token(access_token).await
			.change_context(AuthError::InvalidAccessTokenError)
	}
	async fn get_auth_user_by_access_token(
		&self,
		access_token: &str
	) -> Result<AuthUser, AuthError> {
		get_auth_user_by_access_token(&*self.api_provider, access_token).await
	}
	async fn get_tokens(&self, code: &str) -> Result<GetTokensOutput, AuthError> {
		self.api_provider
			.get_tokens(code).await
			.change_context(AuthError::Generic("Invalid auth code".to_string()))
	}
}
