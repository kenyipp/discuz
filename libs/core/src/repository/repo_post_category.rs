pub use crate::repository::{
	database::db_post_category::{
		CategoryFilter, CreateCategoryInput, DbPostCategory, DbPostCategoryTrait, DefPostCategory,
		InputCategoryList, UpdateCategoryInput,
	},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoPostCategory {
	db_post_category: DbPostCategory,
}

impl RepoPostCategory {
	pub fn new(db_post_category: DbPostCategory) -> RepoPostCategory {
		RepoPostCategory { db_post_category }
	}
}

#[async_trait]
pub trait RepoPostCategoryTrait {
	async fn list(&self, input: &InputCategoryList) -> Result<Vec<DefPostCategory>, RepoError>;
	async fn count(&self, &filter: &CategoryFilter) -> Result<u64, RepoError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<DefPostCategory>, RepoError>;
	async fn find_by_slug(&self, slug: &str) -> Result<Option<DefPostCategory>, RepoError>;
	async fn create(&self, input: &CreateCategoryInput) -> Result<String, RepoError>;
	async fn update(&self, input: &UpdateCategoryInput) -> Result<DefPostCategory, RepoError>;
	async fn delete(&self, id: &str) -> Result<(), RepoError>;
}

#[async_trait]
impl RepoPostCategoryTrait for RepoPostCategory {
	async fn list(&self, input: &InputCategoryList) -> Result<Vec<DefPostCategory>, RepoError> {
		self.db_post_category
			.list(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn count(&self, filter: &CategoryFilter) -> Result<u64, RepoError> {
		self.db_post_category
			.count(filter)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<DefPostCategory>, RepoError> {
		self.db_post_category
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_by_slug(&self, slug: &str) -> Result<Option<DefPostCategory>, RepoError> {
		self.db_post_category
			.find_by_slug(slug)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreateCategoryInput) -> Result<String, RepoError> {
		self.db_post_category
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update(&self, input: &UpdateCategoryInput) -> Result<DefPostCategory, RepoError> {
		self.db_post_category
			.update(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn delete(&self, id: &str) -> Result<(), RepoError> {
		self.db_post_category
			.delete(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)?;
		Ok(())
	}
}
