use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProviderError {
	Generic(String),
	InvalidCredentials,
	InvalidUserCode,
	InvalidAccessToken,
}

impl fmt::Display for ProviderError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Provider Error")
	}
}

impl Error for ProviderError {}
