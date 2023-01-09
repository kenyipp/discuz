use std::{ fmt, error::Error };

#[derive(Debug)]
pub enum UserError {
	Generic,
	InvalidCode,
	InvalidCredentials,
}

impl fmt::Display for UserError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.write_str("User Error")
	}
}

impl Error for UserError {}
