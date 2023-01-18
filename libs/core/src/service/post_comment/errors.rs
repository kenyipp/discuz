use derive_more::{Display, Error};
use strum_macros::EnumProperty;

#[derive(Debug, Display, EnumProperty, Error)]
pub enum PostCommentError {
	#[display(fmt = "Post Comment Error: Generic {}", _0)]
	#[strum(props(code = "generic"))]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Post Comment Error: Internal Server Error")]
	#[strum(props(code = "internal_server_error"))]
	InternalServerError,
}
