use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ProviderError {
	#[display(fmt = "Provider Error: Generic {}", _0)]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Provider Error: Invalid Credentials")]
	InvalidCredentials,
	#[display(fmt = "Provider Error: Invalid User Code")]
	InvalidUserCode,
	#[display(fmt = "Provider Error: Invalid Access Token")]
	InvalidAccessToken,
}
