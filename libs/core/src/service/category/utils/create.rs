use crate::{
	repository::repo_category::{self, Category, RepoCategory, RepoCategoryTrait},
	service::category::{
		errors::CategoryError,
		utils::update::{execute as updateCategory, UpdateCategoryInput},
	},
};
use error_stack::{Result, ResultExt};
use slugify::slugify;

pub async fn execute(
	repo_category: &RepoCategory,
	input: &CreateCategoryInput,
) -> Result<Category, CategoryError> {
	let slug = slugify!(&input.name);
	let category = repo_category
		.find_by_slug(&slug)
		.await
		.change_context(CategoryError::InternalServerError)?;

	if let Some(category) = category {
		if category.status_id == "D" {
			let category = update_deleted_category(repo_category, &category.id, input).await?;
			Ok(category)
		} else {
			let error = CategoryError::DuplicateCategoryError {
				name: input.name.to_owned(),
				detail: Some(format!("The post category {} already exists", &input.name)),
			};
			Err(error.into())
		}
	} else {
		let category = create_category(repo_category, input).await?;
		Ok(category)
	}
}

pub async fn create_category(
	repo_category: &RepoCategory,
	input: &CreateCategoryInput,
) -> Result<Category, CategoryError> {
	let input = repo_category::CreateCategoryInput {
		name: input.name.to_owned(),
		slug: slugify!(&input.name),
		parent_id: input.parent_id.to_owned(),
		description: input.description.to_owned(),
		postable: input.postable.to_owned(),
		level: input.level.to_owned(),
		user_id: input.user_id.to_owned(),
	};
	let id = repo_category
		.create(&input)
		.await
		.change_context(CategoryError::InternalServerError)?;

	let category = repo_category
		.find_by_id(&id)
		.await
		.change_context(CategoryError::InternalServerError)?
		.ok_or(CategoryError::InternalServerError)?;

	Ok(category)
}

pub async fn update_deleted_category(
	repo_category: &RepoCategory,
	id: &str,
	input: &CreateCategoryInput,
) -> Result<Category, CategoryError> {
	let CreateCategoryInput {
		name,
		description,
		parent_id,
		user_id,
		postable,
		level,
	} = input;

	let input = UpdateCategoryInput {
		id: id.to_owned(),
		name: name.to_owned(),
		description: description.to_owned(),
		postable: postable.to_owned(),
		level: level.to_owned(),
		user_id: user_id.to_owned(),
		parent_id: parent_id.to_owned(),
		status_id: Some("A".to_owned()),
	};

	let category = updateCategory(repo_category, &input).await?;
	Ok(category)
}

#[derive(Debug, Clone)]
pub struct CreateCategoryInput {
	pub name: String,
	pub description: Option<String>,
	pub postable: bool,
	pub level: i32,
	pub parent_id: Option<String>,
	pub user_id: Option<String>,
}
