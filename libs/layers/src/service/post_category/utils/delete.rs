use crate::{
	constants::UNCLASSIFIED_CATEGORY_ID,
	repository::repo_post_category::{RepoPostCategory, RepoPostCategoryTrait},
	service::post_category::errors::PostCategoryError,
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	id: &str,
) -> Result<(), PostCategoryError> {
	repo_post_category
		.delete(id)
		.await
		.change_context(PostCategoryError::Generic(
			"Unable to delete the category".to_string(),
		))?;
	Ok(())
}
