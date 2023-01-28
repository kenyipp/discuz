use crate::{
	repository::repo_post_reply::{RepoPostReply, RepoPostReplyTrait},
	service::post_reply::errors::PostReplyError,
};
use error_stack::{FutureExt, Result};

pub async fn execute(repo_post_reply: &RepoPostReply, id: i32) -> Result<(), PostReplyError> {
	repo_post_reply
		.delete(id)
		.change_context(PostReplyError::InternalServerError)
		.await?;
	Ok(())
}
