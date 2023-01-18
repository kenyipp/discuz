use crate::{
	repository::repo_file::{RepoFile, RepoFileTrait},
	service::file::{errors::FileError, provider::api_provider::ApiS3Trait},
};
use error_stack::{FutureExt, Result, ResultExt};

pub async fn get_file(
	repo_file: &RepoFile,
	api_provider: &dyn ApiS3Trait,
	id: &str,
) -> Result<String, FileError> {
	let file = repo_file
		.find_by_id(id)
		.change_context(FileError::Generic(
			"Unable to retrieve data from repository".to_owned(),
		))
		.await?
		.ok_or(FileError::UnableToRetrieveFile)?;

	let url = api_provider
		.get_file(&file.id, None)
		.await
		.change_context(FileError::UnableToRetrieveFile)?;

	Ok(url)
}
