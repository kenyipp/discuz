pub use crate::repository::{
	database::category::{
		Category, CategoryFilter, CreateCategoryInput, DbCategory, DbCategoryTrait,
		ListCategoryInput, UpdateCategoryInput,
	},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoCategory {
	category: DbCategory,
}

impl RepoCategory {
	pub fn new(category: DbCategory) -> RepoCategory {
		RepoCategory { category }
	}
}

#[async_trait]
pub trait RepoCategoryTrait {
	async fn list(&self, input: &ListCategoryInput) -> Result<Vec<Category>, RepoError>;
	async fn count(&self, &filter: &CategoryFilter) -> Result<u64, RepoError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<Category>, RepoError>;
	async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, RepoError>;
	async fn create(&self, input: &CreateCategoryInput) -> Result<String, RepoError>;
	async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, RepoError>;
	async fn delete(&self, id: &str) -> Result<(), RepoError>;
}

#[async_trait]
impl RepoCategoryTrait for RepoCategory {
	async fn list(&self, input: &ListCategoryInput) -> Result<Vec<Category>, RepoError> {
		self.category
			.list(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn count(&self, filter: &CategoryFilter) -> Result<u64, RepoError> {
		self.category
			.count(filter)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<Category>, RepoError> {
		self.category
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, RepoError> {
		self.category
			.find_by_slug(slug)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreateCategoryInput) -> Result<String, RepoError> {
		self.category
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, RepoError> {
		self.category
			.update(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn delete(&self, id: &str) -> Result<(), RepoError> {
		self.category
			.delete(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)?;
		Ok(())
	}
}
