use std::{fmt::Debug, sync::Arc};

use aws_config::SdkConfig;
use discuz_utils::config::{get_config, S3};
use error_stack::Result;

use aws_sdk_s3::Client;

use crate::service::file::provider::{
	errors::ProviderError,
	utils::{delete_file, get_file, get_upload_url},
};

pub use crate::service::file::provider::utils::{GetUploadUrlInput, GetUploadUrlResponse};

#[derive(Debug, Clone)]
pub struct ApiS3 {
	pub client: Client,
	pub bucket: String,
}

impl ApiS3 {
	pub fn new(sdk_config: &Arc<SdkConfig>) -> ApiS3 {
		let config = get_config().clone();
		let client = Client::new(sdk_config);
		let S3 { bucket } = config.amazon.s3;
		ApiS3 { client, bucket }
	}
}

#[async_trait]
pub trait ApiS3Trait: Sync + Send + Debug {
	async fn get_file(&self, key: &str, expires_in: Option<u64>) -> Result<String, ProviderError>;
	async fn delete_file(&self, key: &str) -> Result<(), ProviderError>;
	async fn get_upload_url(
		&self,
		input: &GetUploadUrlInput,
	) -> Result<GetUploadUrlResponse, ProviderError>;
}

#[async_trait]
impl ApiS3Trait for ApiS3 {
	async fn get_file(&self, key: &str, expires_in: Option<u64>) -> Result<String, ProviderError> {
		get_file(&self.client, &self.bucket, key, expires_in).await
	}

	async fn delete_file(&self, key: &str) -> Result<(), ProviderError> {
		delete_file(&self.client, &self.bucket, key).await
	}

	async fn get_upload_url(
		&self,
		input: &GetUploadUrlInput,
	) -> Result<GetUploadUrlResponse, ProviderError> {
		get_upload_url(&self.client, &self.bucket, input).await
	}
}
