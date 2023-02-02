use crate::{
	repository::repo_category::{Category, RepoCategory, RepoCategoryTrait},
	service::category::errors::CategoryError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(
	repo_category: &RepoCategory,
	slug: &str,
) -> Result<Option<Category>, CategoryError> {
	let category = repo_category.find_by_slug(slug).await.map_err(|error| {
		error!("{:#?}", error);
		CategoryError::InternalServerError
	})?;

	if let Some(category) = category {
		if category.status_id == "A" {
			return Ok(Some(category));
		}
	}
	Ok(None)
}
