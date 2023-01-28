use crate::{
	repository::repo_post_reply::{PostReply, RepoPostReply, RepoPostReplyTrait},
	service::post_reply::errors::PostReplyError,
};
use error_stack::{FutureExt, Result};

pub async fn execute(
	repo_post_reply: &RepoPostReply,
	id: i32,
) -> Result<Option<PostReply>, PostReplyError> {
	let post_reply = repo_post_reply
		.find_by_id(id)
		.change_context(PostReplyError::InternalServerError)
		.await?;
	Ok(post_reply)
}
