use crate::service::file::provider::errors::ProviderError;
use aws_sdk_s3::Client;
use error_stack::Result;

pub async fn delete_file(client: &Client, bucket: &str, key: &str) -> Result<(), ProviderError> {
	client
		.delete_object()
		.bucket(bucket)
		.key(key)
		.send()
		.await
		.ok();

	Ok(())
}
