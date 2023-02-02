pub use crate::repository::repo_post::CreateReplyInput;
use crate::{
	repository::repo_post::{PostReply, RepoPost, RepoPostTrait},
	service::post::{
		errors::PostError,
		utils::{find_by_id, find_reply_by_id},
	},
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_post: &RepoPost,
	input: &CreateReplyInput,
) -> Result<PostReply, PostError> {
	let post = find_by_id::execute(repo_post, input.post_id)
		.await?
		.ok_or(PostError::PostNotExistError)?;

	if post.comment_count + 1 > post.max_comment_count {
		return Err(PostError::MaximumReplyError.into());
	}

	let reply_id = repo_post
		.create_reply(input)
		.await
		.change_context(PostError::InternalServerError)?;

	let post_reply = find_reply_by_id::execute(repo_post, reply_id)
		.await?
		.ok_or(PostError::InternalServerError)?;

	Ok(post_reply)
}
