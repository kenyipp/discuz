use std::{ fmt, error::Error };

#[derive(Debug)]
pub enum AuthError {
	Generic(String),
	InvalidAccessTokenError,
}

impl fmt::Display for AuthError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.write_str("Auth Error")
	}
}

impl Error for AuthError {}
