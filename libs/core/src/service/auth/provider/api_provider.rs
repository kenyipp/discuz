use std::{fmt::Debug, sync::Arc};

use aws_config::SdkConfig;
use aws_sdk_cognitoidentityprovider::{output::GetUserOutput, Client};
use discuz_utils::config::{get_config, Amazon, Cognito};
use error_stack::Result;

use crate::service::auth::provider::{
	errors::ProviderError,
	utils::{get_tokens, get_user_by_access_token, validate_access_token},
};

pub use get_tokens::GetTokensOutput;

#[derive(Debug, Clone)]
pub struct ApiCognito {
	pub domain: String,
	pub user_pool_id: String,
	pub client_id: String,
	pub redirect_uri: String,
	pub client: Client,
}

impl ApiCognito {
	pub fn new(sdk_config: &Arc<SdkConfig>) -> ApiCognito {
		let config = get_config().clone();
		let client = Client::new(sdk_config);
		let Cognito {
			user_pool_id,
			domain,
			client_id,
			redirect_uri,
		} = config.amazon.cognito;
		ApiCognito {
			domain,
			user_pool_id,
			client_id,
			redirect_uri,
			client,
		}
	}
}

#[async_trait]
pub trait ApiCognitoTrait: Sync + Send + Debug {
	async fn get_user_by_access_token(
		&self,
		access_token: &str,
	) -> Result<GetUserOutput, ProviderError>;
	async fn validate_access_token(&self, access_token: &str) -> Result<String, ProviderError>;
	async fn get_tokens(&self, code: &str) -> Result<GetTokensOutput, ProviderError>;
}

#[async_trait]
impl ApiCognitoTrait for ApiCognito {
	async fn get_user_by_access_token(
		&self,
		access_token: &str,
	) -> Result<GetUserOutput, ProviderError> {
		get_user_by_access_token::execute(&self.client, access_token).await
	}
	async fn validate_access_token(&self, access_token: &str) -> Result<String, ProviderError> {
		let config = get_config();
		let Amazon { region, .. } = config.amazon.clone();
		validate_access_token::execute(&region, &self.user_pool_id, access_token).await
	}
	async fn get_tokens(&self, code: &str) -> Result<GetTokensOutput, ProviderError> {
		get_tokens::execute(&self.domain, &self.client_id, &self.redirect_uri, code).await
	}
}
