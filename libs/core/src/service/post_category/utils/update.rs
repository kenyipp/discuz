use crate::{
	repository::repo_post_category::{
		self, DefPostCategory, RepoPostCategory, RepoPostCategoryTrait,
	},
	service::post_category::{errors::PostCategoryError, utils::find_by_id::execute as find_by_id},
};
use error_stack::{Result, ResultExt};
use slugify::slugify;

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	input: &UpdateCategoryInput,
) -> Result<DefPostCategory, PostCategoryError> {
	let post_category = find_by_id(repo_post_category, &input.id)
		.await?
		.ok_or(PostCategoryError::CategoryNotExistError)?;

	let input = repo_post_category::UpdateCategoryInput {
		id: input.id.to_owned(),
		name: input.name.to_owned(),
		slug: slugify!(&input.name),
		parent_id: input.parent_id.to_owned(),
		description: input.description.to_owned(),
		user_id: input.user_id.to_owned(),
		status_id: input
			.status_id
			.to_owned()
			.unwrap_or(post_category.status_id),
	};

	let post_category = repo_post_category
		.update(&input)
		.await
		.change_context(PostCategoryError::InternalServerError)?;

	Ok(post_category)
}

#[derive(Debug, Clone)]
pub struct UpdateCategoryInput {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	pub parent_id: Option<String>,
	pub user_id: Option<String>,
	pub status_id: Option<String>,
}
