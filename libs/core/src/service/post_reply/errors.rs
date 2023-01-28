use derive_more::{Display, Error};
use strum_macros::EnumProperty;

#[derive(Debug, Display, EnumProperty, Error)]
pub enum PostReplyError {
	#[display(fmt = "Post Comment Error: Generic {_0}")]
	#[strum(props(code = "post_reply_generic"))]
	Generic(#[error(not(source))] String),
	#[display(fmt = "Post Comment Error: Invalid Post Id")]
	#[strum(props(code = "post_reply_invalid_post"))]
	InvalidPostError,
	#[display(fmt = "Post Comment Error: Maximum comment reached")]
	#[strum(props(code = "post_reply_maximum_comment"))]
	MaximumCommentError,
	#[display(fmt = "Post Comment Error: Internal Server Error")]
	#[strum(props(code = "post_reply_internal_server_error"))]
	InternalServerError,
}
