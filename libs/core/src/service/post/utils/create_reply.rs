use crate::{
	repository::repo_post::{
		CreateReplyInput as RepoCreateReplyInput, PostReply, RepoPost, RepoPostTrait,
	},
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

	let next_no_of_reply = post.comment_count + 2;

	if next_no_of_reply > post.max_comment_count + 1 {
		return Err(PostError::MaximumReplyError.into());
	}

	let input = RepoCreateReplyInput {
		post_id: input.post_id,
		quote_reply_id: input.quote_reply_id,
		no_of_reply: next_no_of_reply as u32,
		content: input.content.to_owned(),
		user_id: input.user_id.to_owned(),
	};

	let reply_id = repo_post
		.create_reply(&input)
		.await
		.change_context(PostError::InternalServerError)?;

	let post_reply = find_reply_by_id::execute(repo_post, reply_id)
		.await?
		.ok_or(PostError::InternalServerError)?;

	Ok(post_reply)
}

pub struct CreateReplyInput {
	pub post_id: i32,
	pub quote_reply_id: Option<i32>,
	pub content: String,
	pub user_id: String,
}
