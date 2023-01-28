use derive_more::{Display, Error};
use strum_macros::EnumProperty;

#[derive(Debug, Display, EnumProperty, Error)]
pub enum PostError {
	#[display(fmt = "Post Error: Generic {_0}")]
	#[strum(props(code = "post_generic"))]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Post Error: Internal Server Error")]
	#[strum(props(code = "post_internal_server_error"))]
	InternalServerError,
	#[display(fmt = "Post Error: Target post not exist")]
	#[strum(props(code = "post_not_exist"))]
	PostNotExistError,
}
