use crate::{
	repository::repo_file::RepoFile,
	service::file::{
		constants::FileType,
		errors::FileError,
		provider::api_provider::ApiS3Trait,
		utils::{
			get_file::get_file,
			get_upload_url::{get_upload_url, GetUploadUrlResponse},
		},
	},
};
use error_stack::Result;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct FileService {
	pub repo_file: RepoFile,
	pub api_provider: Arc<dyn ApiS3Trait>,
	pub bucket: String,
}

#[async_trait]
pub trait FileServiceTrait: Sync + Send + Debug {
	// Get the private file by file id
	async fn get_file(&self, key: &str) -> Result<String, FileError>;
	// Get the presigned upload url
	async fn get_upload_url(&self, file_type: &FileType)
		-> Result<GetUploadUrlResponse, FileError>;
}

#[async_trait]
impl FileServiceTrait for FileService {
	async fn get_file(&self, key: &str) -> Result<String, FileError> {
		get_file(&self.repo_file, &*self.api_provider, key).await
	}
	async fn get_upload_url(
		&self,
		file_type: &FileType,
	) -> Result<GetUploadUrlResponse, FileError> {
		get_upload_url(&*self.api_provider, file_type).await
	}
}
