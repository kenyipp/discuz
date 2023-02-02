use error_stack::Result;
use std::fmt::Debug;

pub use crate::{
	repository::repo_category::Category,
	service::category::utils::{
		create::CreateCategoryInput,
		get_categories::{GetCategoriesResponse, InputCategoryList},
		update::UpdateCategoryInput,
	},
};

use crate::{
	repository::repo_category::RepoCategory,
	service::category::{errors::CategoryError, utils},
};

#[derive(Debug, Clone)]
pub struct CategoryService {
	pub repo_category: RepoCategory,
}

#[async_trait]
pub trait CategoryServiceTrait: Sync + Send + Debug {
	async fn get_categories(
		&self,
		input: Option<&InputCategoryList>,
	) -> Result<GetCategoriesResponse, CategoryError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<Category>, CategoryError>;
	async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, CategoryError>;
	async fn create(&self, input: &CreateCategoryInput) -> Result<Category, CategoryError>;
	async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, CategoryError>;
	async fn delete(&self, id: &str) -> Result<(), CategoryError>;
}

#[async_trait]
impl CategoryServiceTrait for CategoryService {
	async fn get_categories(
		&self,
		input: Option<&InputCategoryList>,
	) -> Result<GetCategoriesResponse, CategoryError> {
		utils::get_categories::execute(&self.repo_category, input).await
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<Category>, CategoryError> {
		utils::find_by_id::execute(&self.repo_category, id).await
	}

	async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, CategoryError> {
		utils::find_by_slug::execute(&self.repo_category, slug).await
	}

	async fn create(&self, input: &CreateCategoryInput) -> Result<Category, CategoryError> {
		utils::create::execute(&self.repo_category, input).await
	}

	async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, CategoryError> {
		utils::update::execute(&self.repo_category, input).await
	}

	async fn delete(&self, id: &str) -> Result<(), CategoryError> {
		utils::delete::execute(&self.repo_category, id).await
	}
}
