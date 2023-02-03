use crate::{
	repository::repo_category::{self, Category, RepoCategory, RepoCategoryTrait},
	service::category::{errors::CategoryError, utils::find_by_id::execute as find_by_id},
};
use error_stack::{Result, ResultExt};
use slugify::slugify;

pub async fn execute(
	repo_category: &RepoCategory,
	input: &UpdateCategoryInput,
) -> Result<Category, CategoryError> {
	let category = find_by_id(repo_category, &input.id)
		.await?
		.ok_or(CategoryError::CategoryNotExistError)?;

	let input = repo_category::UpdateCategoryInput {
		id: input.id.to_owned(),
		name: input.name.to_owned(),
		slug: slugify!(&input.name),
		parent_id: input.parent_id.to_owned(),
		description: input.description.to_owned(),
		postable: input.postable.to_owned(),
		sort_index: input.sort_index.to_owned(),
		level: input.level.to_owned(),
		user_id: input.user_id.to_owned(),
		status_id: input.status_id.to_owned().unwrap_or(category.status_id),
	};

	let category = repo_category
		.update(&input)
		.await
		.change_context(CategoryError::InternalServerError)?;

	Ok(category)
}

#[derive(Debug, Clone)]
pub struct UpdateCategoryInput {
	pub id: String,
	pub name: String,
	pub description: Option<String>,
	pub postable: bool,
	pub level: u32,
	pub sort_index: i32,
	pub parent_id: Option<String>,
	pub user_id: Option<String>,
	pub status_id: Option<String>,
}
