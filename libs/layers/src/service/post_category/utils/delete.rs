use crate::{
	repository::repo_post_category::{RepoPostCategory, RepoPostCategoryTrait},
	service::post_category::{errors::PostCategoryError, utils::find_by_id::execute as find_by_id},
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	id: &str,
) -> Result<(), PostCategoryError> {
	find_by_id(repo_post_category, id)
		.await?
		.ok_or_else(|| PostCategoryError::CategoryNotExistError)?;

	repo_post_category
		.delete(id)
		.await
		.change_context(PostCategoryError::InternalServerError)?;
	Ok(())
}
