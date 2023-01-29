use std::collections::HashMap;

use error_stack::{IntoReport, Report, Result, ResultExt};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use tokio::sync::Mutex;

use crate::service::auth::provider::errors::ProviderError;

lazy_static! {
	static ref JWK_CACHE: Mutex<HashMap<String, Vec<Jwk>>> = Mutex::new(HashMap::new());
}

pub async fn execute(
	region: &str,
	user_pool_id: &str,
	access_token: &str,
) -> Result<String, ProviderError> {
	// Download the Jwk keys by the region and user pool id
	let jwk_url = get_jwk_url(region, user_pool_id);
	let jwk_keys = get_json_web_tokens(&jwk_url).await?;

	let header = decode_header(access_token)
		.into_report()
		.change_context(ProviderError::InvalidAccessToken)?;

	let jwk = jwk_keys
		.iter()
		.enumerate()
		.find(|jwk| jwk.1.kid == header.to_owned().kid.unwrap());

	let decoding_key = match jwk {
		Some(jwk) => {
			let Jwk { n, e, .. } = jwk.1;
			match DecodingKey::from_rsa_components(n, e) {
				Ok(decoding_key) => decoding_key,
				Err(_) => {
					return Err(Report::new(ProviderError::Generic(
						"Unable to create decoding key from the selected Jwk".to_string(),
					)));
				}
			}
		}
		None => {
			return Err(
				Report::new(ProviderError::InvalidAccessToken).attach_printable(
					"Unable to retrieve the associated json web key for this access token",
				),
			);
		}
	};

	let decoded = decode::<CognitoPayload>(
		access_token,
		&decoding_key,
		&Validation::new(Algorithm::RS256),
	)
	.into_report()
	.change_context(ProviderError::InvalidAccessToken)?;

	Ok(decoded.claims.sub)
}

async fn get_json_web_tokens(jwk_url: &str) -> Result<Vec<Jwk>, ProviderError> {
	let mut cache = JWK_CACHE.lock().await;
	if !cache.contains_key(jwk_url) {
		let response: GetJsonWebTokensResponse = reqwest::get(jwk_url)
			.await
			.into_report()
			.change_context(ProviderError::InvalidCredentials)
			.attach_printable("The JWK Url is not valid url")?
			.json()
			.await
			.into_report()
			.change_context(ProviderError::Generic(
				"Unable to convert the API response to json object".to_string(),
			))?;
		cache.insert(jwk_url.to_owned(), response.keys);
	}
	Ok(cache.get(jwk_url).unwrap().clone())
}

fn get_jwk_url(region: &str, user_pool_id: &str) -> String {
	format!("https://cognito-idp.{region}.amazonaws.com/{user_pool_id}/.well-known/jwks.json")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwk {
	pub alg: String,
	pub e: String,
	pub kid: String,
	pub kty: String,
	pub n: String,
	#[serde(rename = "use")]
	pub jwk_use: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetJsonWebTokensResponse {
	keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CognitoPayload {
	pub sub: String,
	#[serde(rename = "cognito:groups")]
	pub cognito_groups: Vec<String>,
	pub iss: String,
	pub version: u32,
	pub client_id: String,
	pub origin_jti: String,
	pub token_use: String,
	pub scope: String,
	pub auth_time: u32,
	pub exp: u32,
	pub jti: String,
	pub username: String,
}
