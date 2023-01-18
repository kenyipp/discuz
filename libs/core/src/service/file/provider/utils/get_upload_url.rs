use crate::{constants::BYTES_OF_1_MB, service::file::provider::errors::ProviderError};
use aws_sdk_s3::{model::ObjectCannedAcl, presigning::config::PresigningConfig, Client};
use error_stack::{IntoReport, Result, ResultExt};
use serde::Serialize;
use std::time::Duration;
use uuid::Uuid;

pub async fn get_upload_url(
	client: &Client,
	bucket: &str,
	input: &GetUploadUrlInput,
) -> Result<GetUploadUrlResponse, ProviderError> {
	let file_id = &Uuid::new_v4().to_string();

	let presigning_config = PresigningConfig::expires_in(Duration::from_secs(input.expires_in))
		.into_report()
		.change_context(ProviderError::InvalidPresigningConfig)?;

	let upload_uri = client
		.put_object()
		.bucket(bucket)
		.key(file_id)
		.acl(if input.is_public {
			ObjectCannedAcl::PublicRead
		} else {
			ObjectCannedAcl::Private
		})
		.content_length(input.max_file_size)
		.presigned(presigning_config)
		.await
		.into_report()
		.change_context(ProviderError::InvalidPresignedUrlRequest)?
		.uri()
		.to_string();

	let public_uri = if input.is_public {
		Some(format!("https://{}.s3.amazonaws.com/{}", bucket, file_id))
	} else {
		None
	};

	Ok(GetUploadUrlResponse {
		file_id: file_id.to_string(),
		upload_uri,
		public_uri,
	})
}

#[derive(Debug, Clone)]
pub struct GetUploadUrlInput {
	pub is_public: bool,
	pub expires_in: u64,
	pub max_file_size: i64,
}

impl Default for GetUploadUrlInput {
	fn default() -> Self {
		Self {
			is_public: false,
			expires_in: 60 * 15,
			max_file_size: 5 * BYTES_OF_1_MB, // 5mb
		}
	}
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUploadUrlResponse {
	pub file_id: String,
	pub upload_uri: String,
	pub public_uri: Option<String>,
}
