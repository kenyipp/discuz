use error_stack::{IntoReport, Result, ResultExt};
use reqwest;
use serde::{Deserialize, Serialize};

use super::super::errors::ProviderError;

pub async fn get_tokens(
	domain: &str,
	client_id: &str,
	redirect_uri: &str,
	code: &str,
) -> Result<GetTokensOutput, ProviderError> {
	let params = vec![
		("grant_type", "authorization_code"),
		("client_id", client_id),
		("code", code),
		("redirect_uri", redirect_uri),
	];

	let url = &format!("https://{domain}/oauth2/token");

	let response: GetTokensOutput = reqwest::Client::new()
		.post(url)
		.form(&params)
		.send()
		.await
		.into_report()
		.change_context(ProviderError::Generic("Invalid auth code".to_string()))?
		.json()
		.await
		.into_report()
		.change_context(ProviderError::Generic("Unexpected error".to_string()))?;

	Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokensOutput {
	pub id_token: String,
	pub access_token: String,
	pub refresh_token: String,
	pub expires_in: u32,
	pub token_type: String,
}
