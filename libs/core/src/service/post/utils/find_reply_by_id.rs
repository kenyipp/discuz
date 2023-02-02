use crate::{
	repository::repo_post::{PostReply, RepoPost, RepoPostTrait},
	service::post::errors::PostError,
};
use error_stack::{FutureExt, Result};

pub async fn execute(repo_post: &RepoPost, id: i32) -> Result<Option<PostReply>, PostError> {
	let post_reply = repo_post
		.find_reply_by_id(id)
		.change_context(PostError::InternalServerError)
		.await?;
	Ok(post_reply)
}
