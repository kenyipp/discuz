use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
	#[display(fmt = "User Error: Generic")]
	Generic,
	#[display(fmt = "User Error: Invalid Code")]
	InvalidCode,
	#[display(fmt = "User Error: Invalid Credentials")]
	InvalidCredentials,
}
