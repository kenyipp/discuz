use derive_more::{Display, Error};
use strum_macros::EnumProperty;

#[derive(Debug, Display, EnumProperty, Error)]
pub enum UserBanHistoryError {
	#[display(fmt = "User Ban History Error: Generic {_0}")]
	#[strum(props(code = "user_ban_history_generic"))]
	Generic(#[error(not(source))] String),
	#[display(fmt = "User Ban History Error: Target history not exist")]
	#[strum(props(code = "user_ban_history_not_exist"))]
	UserBanHistoryNotExistError,
	#[display(fmt = "User Ban History Error: Internal Server Error")]
	#[strum(props(code = "user_ban_history_internal_server_error"))]
	InternalServerError,
}
