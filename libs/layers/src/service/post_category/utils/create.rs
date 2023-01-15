use crate::{
	repository::repo_post_category::{
		self, DefPostCategory, RepoPostCategory, RepoPostCategoryTrait,
	},
	service::post_category::{
		errors::PostCategoryError,
		utils::update::{execute as updateCategory, UpdateCategoryInput},
	},
};
use error_stack::{Result, ResultExt};
use slugify::slugify;

pub async fn execute(
	repo_post_category: &RepoPostCategory,
	input: &CreateCategoryInput,
) -> Result<DefPostCategory, PostCategoryError> {
	let slug = slugify!(&input.name);
	let post_category = repo_post_category
		.find_by_slug(&slug)
		.await
		.change_context(PostCategoryError::InternalServerError)?;

	if let Some(post_category) = post_category {
		if post_category.status_id == "D" {
			let post_category =
				update_deleted_post_category(repo_post_category, &post_category.id, input).await?;
			Ok(post_category)
		} else {
			let error = PostCategoryError::DuplicateCategoryError {
				name: input.name.to_owned(),
				detail: Some(format!("The post category {} already exists", &input.name)),
			};
			Err(error.into())
		}
	} else {
		let post_category = create_post_category(repo_post_category, input).await?;
		Ok(post_category)
	}
}

pub async fn create_post_category(
	repo_post_category: &RepoPostCategory,
	input: &CreateCategoryInput,
) -> Result<DefPostCategory, PostCategoryError> {
	let input = repo_post_category::CreateCategoryInput {
		name: input.name.to_owned(),
		slug: slugify!(&input.name),
		description: input.description.to_owned(),
		user_id: input.user_id.to_owned(),
	};
	let id = repo_post_category
		.create(&input)
		.await
		.change_context(PostCategoryError::InternalServerError)?;

	let post_category = repo_post_category
		.find_by_id(&id)
		.await
		.change_context(PostCategoryError::InternalServerError)?
		.ok_or_else(|| PostCategoryError::InternalServerError)?;

	Ok(post_category)
}

pub async fn update_deleted_post_category(
	repo_post_category: &RepoPostCategory,
	id: &str,
	input: &CreateCategoryInput,
) -> Result<DefPostCategory, PostCategoryError> {
	let CreateCategoryInput {
		name,
		description,
		user_id,
	} = input;

	let input = UpdateCategoryInput {
		id: id.to_owned(),
		name: name.to_owned(),
		description: description.to_owned(),
		user_id: user_id.to_owned(),
		status_id: Some("A".to_owned()),
	};

	let post_category = updateCategory(repo_post_category, &input).await?;
	Ok(post_category)
}

#[derive(Debug, Clone)]
pub struct CreateCategoryInput {
	pub name: String,
	pub description: Option<String>,
	pub user_id: Option<String>,
}
