use crate::{
	repository::repo_category::{Category, RepoCategory, RepoCategoryTrait},
	service::category::errors::CategoryError,
};
use error_stack::Result;
use tracing::error;

pub async fn execute(
	repo_category: &RepoCategory,
	id: &str,
) -> Result<Option<Category>, CategoryError> {
	let category = repo_category.find_by_id(id).await.map_err(|error| {
		error!("{:#?}", error);
		CategoryError::InternalServerError
	})?;
	Ok(category)
}
