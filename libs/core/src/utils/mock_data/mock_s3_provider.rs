use crate::service::file::{
	provider::{
		api_provider::{ApiS3Trait, GetUploadUrlInput},
		errors::ProviderError,
	},
	utils::get_upload_url::GetUploadUrlResponse,
};
use error_stack::Result;

#[derive(Debug, Clone)]
pub struct MockApiS3;

#[async_trait]
impl ApiS3Trait for MockApiS3 {
	async fn get_file(
		&self,
		_key: &str,
		_expires_in: Option<u64>,
	) -> Result<String, ProviderError> {
		Ok("FILE".to_owned())
	}

	async fn delete_file(&self, _key: &str) -> Result<(), ProviderError> {
		Ok(())
	}

	async fn get_upload_url(
		&self,
		_input: &GetUploadUrlInput,
	) -> Result<GetUploadUrlResponse, ProviderError> {
		Ok(GetUploadUrlResponse {
			file_id: uuid::Uuid::new_v4().to_string(),
			upload_uri: "UPLOAD_URI".to_owned(),
			public_uri: Some("PUBLIC_URI".to_owned()),
		})
	}
}
