use crate::{
	repository::repo_post_category::{DefPostCategory, RepoPostCategory, RepoPostCategoryTrait},
	service::post_category::errors::PostCategoryError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	slug: &str,
) -> Result<Option<DefPostCategory>, PostCategoryError> {
	let post_category = repo_post_category
		.find_by_slug(slug)
		.await
		.map_err(|error| {
			error!("{:#?}", error);
			PostCategoryError::InternalServerError
		})?;

	if let Some(post_category) = post_category {
		if post_category.status_id == "A" {
			return Ok(Some(post_category));
		}
	}
	Ok(None)
}
