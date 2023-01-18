use crate::service::file::provider::errors::ProviderError;
use aws_sdk_s3::{presigning::config::PresigningConfig, Client};
use error_stack::{IntoReport, Result, ResultExt};
use std::time::Duration;

pub async fn get_file(
	client: &Client,
	bucket: &str,
	key: &str,
	expires_in: Option<u64>,
) -> Result<String, ProviderError> {
	let presigning_config =
		PresigningConfig::expires_in(Duration::from_secs(expires_in.unwrap_or(60 * 60 * 24)))
			.into_report()
			.change_context(ProviderError::InvalidPresigningConfig)?;

	let url = client
		.get_object()
		.bucket(bucket)
		.key(key)
		.presigned(presigning_config)
		.await
		.into_report()
		.change_context(ProviderError::InvalidPresignedUrlRequest)?
		.uri()
		.to_string();

	Ok(url)
}
