pub use crate::repository::repo_post::CreateReplyInput;
use crate::{
	repository::repo_post::{RepoPost, RepoPostTrait},
	service::post::errors::PostError,
};
use error_stack::{FutureExt, Result};

pub async fn execute(repo_post: &RepoPost, id: i32) -> Result<(), PostError> {
	repo_post
		.delete_reply(id)
		.change_context(PostError::InternalServerError)
		.await?;
	Ok(())
}
