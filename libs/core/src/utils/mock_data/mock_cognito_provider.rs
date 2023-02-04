use crate::service::auth::provider::{
	api_provider::{ApiCognitoTrait, GetTokensOutput},
	errors::ProviderError,
};
use ::fake::Fake;
use aws_sdk_cognitoidentityprovider::{model::AttributeType, output::GetUserOutput};
use error_stack::Result;
use fake::faker;
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static! {
	static ref USER_CACHE: Mutex<HashMap<String, GetUserOutput>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct MockApiCognito;

#[async_trait]
impl ApiCognitoTrait for MockApiCognito {
	async fn get_user_by_access_token(
		&self,
		access_token: &str,
	) -> Result<GetUserOutput, ProviderError> {
		if let Some(user) = USER_CACHE.lock().await.get(access_token) {
			return Ok(user.to_owned());
		}

		let name: String = faker::name::en::Name().fake();
		let email: String = faker::internet::en::SafeEmail().fake();
		let picture = format!("https://i.pravatar.cc/250?u={access_token}");

		let sub = AttributeType::builder()
			.set_name(Some("sub".to_owned()))
			.set_value(Some(access_token.to_owned()))
			.build();

		let email_verified = AttributeType::builder()
			.set_name(Some("email_verified".to_owned()))
			.set_value(Some("true".to_owned()))
			.build();

		let name = AttributeType::builder()
			.set_name(Some("name".to_owned()))
			.set_value(Some(name))
			.build();

		let phone_number_verified = AttributeType::builder()
			.set_name(Some("phone_number_verified".to_owned()))
			.set_value(Some("true".to_owned()))
			.build();

		let email = AttributeType::builder()
			.set_name(Some("email".to_owned()))
			.set_value(Some(email))
			.build();

		let picture = AttributeType::builder()
			.set_name(Some("picture".to_owned()))
			.set_value(Some(picture))
			.build();

		let output = GetUserOutput::builder()
			.set_username(Some(access_token.to_owned()))
			.set_user_attributes(Some(vec![
				sub,
				email_verified,
				name,
				phone_number_verified,
				email,
				picture,
			]))
			.build();

		USER_CACHE
			.lock()
			.await
			.insert(access_token.to_owned(), output.to_owned());

		Ok(output.to_owned())
	}

	async fn validate_access_token(&self, access_token: &str) -> Result<String, ProviderError> {
		Ok(access_token.to_owned())
	}

	async fn get_tokens(&self, _code: &str) -> Result<GetTokensOutput, ProviderError> {
		todo!()
	}
}
