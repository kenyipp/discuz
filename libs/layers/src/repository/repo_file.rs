pub use crate::repository::{
	database::db_file::{CreateFileInput, DbFile, DbFileTrait, File, UpdateFileInput},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoFile {
	db_file: DbFile,
}

impl RepoFile {
	pub fn new(db_file: DbFile) -> RepoFile {
		RepoFile { db_file }
	}
}

#[async_trait]
pub trait RepoFileTrait {
	async fn create(&self, input: &CreateFileInput) -> Result<String, RepoError>;
	async fn update(&self, input: &UpdateFileInput) -> Result<(), RepoError>;
	async fn update_status(&self, id: &str, status: &str) -> Result<(), RepoError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<File>, RepoError>;
}

#[async_trait]
impl RepoFileTrait for RepoFile {
	async fn create(&self, input: &CreateFileInput) -> Result<String, RepoError> {
		self.db_file
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update(&self, input: &UpdateFileInput) -> Result<(), RepoError> {
		self.db_file
			.update(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update_status(&self, id: &str, status: &str) -> Result<(), RepoError> {
		self.db_file
			.update_status(id, status)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<File>, RepoError> {
		self.db_file
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}
}
