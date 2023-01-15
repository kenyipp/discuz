use crate::{
	repository::repo_post_category::{
		self, DefPostCategory, RepoPostCategory, RepoPostCategoryTrait,
	},
	service::post_category::errors::PostCategoryError,
};
use error_stack::{Result, ResultExt};
use slugify::slugify;

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	input: &UpdateCategoryInput,
) -> Result<DefPostCategory, PostCategoryError> {
	let input = repo_post_category::UpdateCategoryInput {
		id: input.id.to_owned(),
		name: input.name.to_owned(),
		slug: slugify!(&input.name),
		description: input.description.to_owned(),
		user_id: input.user_id.to_owned(),
	};

	repo_post_category
		.update(&input)
		.await
		.change_context(PostCategoryError::Generic(
			"Unable to create the category".to_owned(),
		))?;

	let post_category = repo_post_category
		.find_by_id(&input.id)
		.await
		.change_context(PostCategoryError::Generic(
			"Unable to update the category".to_owned(),
		))?
		.ok_or(PostCategoryError::Generic(
			"Unable to retrieve the category after updated".to_owned(),
		))?;

	Ok(post_category)
}

#[derive(Debug, Clone)]
pub struct UpdateCategoryInput {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	pub user_id: Option<String>,
}
