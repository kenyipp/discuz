use crate::{
	repository::repo_post_category::{DefPostCategory, RepoPostCategory, RepoPostCategoryTrait},
	service::post_category::errors::PostCategoryError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	id: &str,
) -> Result<Option<DefPostCategory>, PostCategoryError> {
	let post_category = repo_post_category.find_by_id(id).await.map_err(|error| {
		error!("{:#?}", error);
		PostCategoryError::Generic("Unable to retrieve the post category".to_owned())
	})?;

	if let Some(post_category) = post_category {
		if post_category.status_id == "A" {
			return Ok(Some(post_category));
		}
	}
	Ok(None)
}
