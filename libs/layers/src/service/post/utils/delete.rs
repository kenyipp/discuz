use crate::{
	repository::repo_post::{RepoPost, RepoPostTrait},
	service::post::{errors::PostError, utils::find_by_id::execute as find_by_id},
};
use error_stack::{Result, ResultExt};

pub async fn execute(repo_post: &RepoPost, id: i32) -> Result<(), PostError> {
	find_by_id(repo_post, id)
		.await?
		.ok_or(PostError::PostNotExistError)?;

	repo_post
		.delete(id)
		.await
		.change_context(PostError::InternalServerError)?;
	Ok(())
}
