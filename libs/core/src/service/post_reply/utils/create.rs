pub use crate::repository::repo_post_reply::CreateCommentInput;
use crate::{
	repository::repo_post_reply::{PostReply, RepoPostCategoryTrait, RepoPostReply},
	service::{
		post::post_service::PostServiceTrait,
		post_reply::{errors::PostReplyError, utils::find_by_id},
	},
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_post_reply: &RepoPostReply,
	post_service: &dyn PostServiceTrait,
	input: &CreateCommentInput,
) -> Result<PostReply, PostReplyError> {
	let post = post_service
		.find_by_id(input.post_id)
		.await
		.change_context(PostReplyError::InternalServerError)?
		.ok_or(PostReplyError::InvalidPostError)?;

	if post.comment_count + 1 > post.max_comment_count {
		return Err(PostReplyError::MaximumCommentError.into());
	}

	let comment_id = repo_post_reply
		.create(input)
		.await
		.change_context(PostReplyError::InternalServerError)?;

	let post_reply = find_by_id::execute(repo_post_reply, comment_id)
		.await?
		.ok_or(PostReplyError::InternalServerError)?;

	Ok(post_reply)
}
