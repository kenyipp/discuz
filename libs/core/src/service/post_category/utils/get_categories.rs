use error_stack::{Result, ResultExt};
use futures::try_join;

pub use crate::repository::repo_post_category::InputCategoryList;

use crate::{
	repository::repo_post_category::{DefPostCategory, RepoPostCategory, RepoPostCategoryTrait},
	service::post_category::errors::PostCategoryError,
};

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	input: Option<&InputCategoryList>,
) -> Result<GetCategoriesResponse, PostCategoryError> {
	let default_input = InputCategoryList::default();
	let input = input.unwrap_or(&default_input);
	let (data, count) = try_join!(
		repo_post_category.list(&input),
		repo_post_category.count(&input.filter)
	)
	.change_context(PostCategoryError::InternalServerError)?;
	Ok(GetCategoriesResponse { data, count })
}

#[derive(Debug, Clone)]
pub struct GetCategoriesResponse {
	pub data: Vec<DefPostCategory>,
	pub count: u64,
}
