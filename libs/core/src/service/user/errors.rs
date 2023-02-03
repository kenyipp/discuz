use derive_more::{Display, Error};
use strum_macros::EnumProperty;

#[derive(Debug, Display, EnumProperty, Error)]
pub enum UserError {
	#[display(fmt = "Generic")]
	#[strum(props(code = "user_generic"))]
	Generic,
	#[display(fmt = "Invalid Code")]
	#[strum(props(code = "user_invalid_code"))]
	InvalidCode,
	#[display(fmt = "Invalid Credentials")]
	InvalidCredentials,
	#[display(fmt = "Target history not exist")]
	#[strum(props(code = "user_ban_history_not_exist"))]
	UserBanHistoryNotExistError,
	#[display(fmt = "Internal Server Error")]
	#[strum(props(code = "user_internal_server_error"))]
	InternalServerError,
}
