use error_stack::{Result, ResultExt};
use tokio::try_join;

pub use crate::repository::repo_category::ListCategoryInput;

use crate::{
	repository::repo_category::{Category, RepoCategory, RepoCategoryTrait},
	service::category::errors::CategoryError,
};

pub async fn execute(
	repo_category: &RepoCategory,
	input: Option<&ListCategoryInput>,
) -> Result<GetCategoriesResponse, CategoryError> {
	let default_input = ListCategoryInput::default();
	let input = input.unwrap_or(&default_input);
	let (data, count) = try_join!(
		repo_category.list(input),
		repo_category.count(&input.filter)
	)
	.change_context(CategoryError::InternalServerError)?;
	Ok(GetCategoriesResponse { data, count })
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetCategoriesResponse {
	pub data: Vec<Category>,
	pub count: u64,
}
