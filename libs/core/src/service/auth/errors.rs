use chrono::{DateTime, Utc};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthError {
	#[display(fmt = "{_0}")]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Invalid access token")]
	InvalidAccessTokenError,
	#[display(fmt = "Insufficient privileges")]
	InsufficientPrivilegesError,
	#[display(fmt = "User banned")]
	UserBannedError {
		reason: Option<String>,
		ban_time: Option<i32>,
		release_time: Option<DateTime<Utc>>,
	},
	#[display(fmt = "Internal server error")]
	InternalServerError,
}
