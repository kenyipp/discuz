use derive_more::{Display, Error};
use error_stack::{IntoReport, Report, Result, ResultExt};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;

lazy_static! {
	static ref JWK_CACHE: Mutex<HashMap<String, Vec<JWK>>> = Mutex::new(HashMap::new());
}

pub async fn validate_cognito_access_token(
	jwk_url: &str,
	access_token: &str,
) -> Result<Payload, JWTError> {
	let jwk_keys = get_json_web_tokens(jwk_url.to_owned()).await?;

	let header = decode_header(access_token)
		.into_report()
		.change_context(JWTError::InvalidAccessToken)?;

	let jwk = jwk_keys
		.iter()
		.enumerate()
		.find(|jwk| jwk.1.kid == header.to_owned().kid.unwrap());

	let decoding_key = match jwk {
		Some(jwk) => {
			let JWK { n, e, .. } = jwk.1;
			match DecodingKey::from_rsa_components(n, e) {
				Ok(decoding_key) => decoding_key,
				Err(_) => {
					return Err(Report::new(JWTError::InvalidJsonWebKey));
				}
			}
		}
		None => {
			return Err(Report::new(JWTError::InvalidJsonWebKey));
		}
	};

	let decoded = decode::<Payload>(
		access_token,
		&decoding_key,
		&Validation::new(Algorithm::RS256),
	)
	.into_report()
	.change_context(JWTError::InvalidAccessToken)?;

	Ok(decoded.claims)
}

pub async fn get_json_web_tokens(jwk_url: String) -> Result<Vec<JWK>, JWTError> {
	let mut cache = JWK_CACHE.lock().await;
	if !cache.contains_key(&jwk_url) {
		let response: JWKsRes = reqwest::get(&jwk_url)
			.await
			.into_report()
			.change_context(JWTError::InvalidJWTUrl)
			.attach_printable("The JWK Url is not valid url")?
			.json()
			.await
			.into_report()
			.change_context(JWTError::Generic)?;
		cache.insert(jwk_url.to_owned(), response.keys);
	}
	Ok(cache.get(&jwk_url).unwrap().clone())
}

/**
 *
 * Errors
 *
 */
#[derive(Debug, Display, Error)]
pub enum JWTError {
	#[display(fmt = "JWT Error: Unable to verify the access token")]
	Generic,
	InvalidJWTUrl,
	InvalidAccessToken,
	InvalidJsonWebKey,
}

/**
 *
 * Types
 *
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWK {
	pub alg: String,
	pub e: String,
	pub kid: String,
	pub kty: String,
	pub n: String,
	#[serde(rename = "use")]
	pub jwk_use: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JWKsRes {
	keys: Vec<JWK>,
}
