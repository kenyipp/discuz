use crate::{
	constants::BYTES_OF_1_MB,
	service::file::{
		constants::FileType,
		errors::FileError,
		provider::api_provider::{ApiS3Trait, GetUploadUrlInput},
	},
};
use error_stack::{Result, ResultExt};

pub use crate::service::file::provider::api_provider::GetUploadUrlResponse;

pub async fn get_upload_url(
	api_provider: &dyn ApiS3Trait,
	file_type: &FileType,
) -> Result<GetUploadUrlResponse, FileError> {
	let input = match file_type {
		FileType::AvatarUri => GetUploadUrlInput {
			is_public: true,
			max_file_size: BYTES_OF_1_MB * 5,
			..GetUploadUrlInput::default()
		},
		FileType::Attachment => GetUploadUrlInput {
			is_public: true,
			max_file_size: BYTES_OF_1_MB * 10,
			..GetUploadUrlInput::default()
		},
	};
	let response = api_provider
		.get_upload_url(&input)
		.await
		.change_context(FileError::UnableToGetTheUploadUrl)?;
	Ok(response)
}
