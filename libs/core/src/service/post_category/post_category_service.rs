use error_stack::Result;
use std::fmt::Debug;

pub use crate::repository::repo_post_category::DefPostCategory;
pub use crate::service::post_category::utils::{
	create::CreateCategoryInput,
	get_categories::{GetCategoriesResponse, InputCategoryList},
	update::UpdateCategoryInput,
};

use crate::{
	repository::repo_post_category::RepoPostCategory,
	service::post_category::{errors::PostCategoryError, utils},
};

#[derive(Debug, Clone)]
pub struct PostCategoryService {
	pub repo_post_category: RepoPostCategory,
}

#[async_trait]
pub trait PostCategoryServiceTrait: Sync + Send + Debug {
	async fn get_categories(
		&self,
		input: Option<&InputCategoryList>,
	) -> Result<GetCategoriesResponse, PostCategoryError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<DefPostCategory>, PostCategoryError>;
	async fn find_by_slug(&self, slug: &str) -> Result<Option<DefPostCategory>, PostCategoryError>;
	async fn create(
		&self,
		input: &CreateCategoryInput,
	) -> Result<DefPostCategory, PostCategoryError>;
	async fn update(
		&self,
		input: &UpdateCategoryInput,
	) -> Result<DefPostCategory, PostCategoryError>;
	async fn delete(&self, id: &str) -> Result<(), PostCategoryError>;
}

#[async_trait]
impl PostCategoryServiceTrait for PostCategoryService {
	async fn get_categories(
		&self,
		input: Option<&InputCategoryList>,
	) -> Result<GetCategoriesResponse, PostCategoryError> {
		utils::get_categories::execute(&self.repo_post_category, input).await
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<DefPostCategory>, PostCategoryError> {
		utils::find_by_id::execute(&self.repo_post_category, id).await
	}

	async fn find_by_slug(&self, slug: &str) -> Result<Option<DefPostCategory>, PostCategoryError> {
		utils::find_by_slug::execute(&self.repo_post_category, slug).await
	}

	async fn create(
		&self,
		input: &CreateCategoryInput,
	) -> Result<DefPostCategory, PostCategoryError> {
		utils::create::execute(&self.repo_post_category, input).await
	}

	async fn update(
		&self,
		input: &UpdateCategoryInput,
	) -> Result<DefPostCategory, PostCategoryError> {
		utils::update::execute(&self.repo_post_category, input).await
	}

	async fn delete(&self, id: &str) -> Result<(), PostCategoryError> {
		utils::delete::execute(&self.repo_post_category, id).await
	}
}
