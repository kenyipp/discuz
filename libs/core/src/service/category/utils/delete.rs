use crate::{
	repository::repo_category::{RepoCategory, RepoCategoryTrait},
	service::category::{errors::CategoryError, utils::find_by_id::execute as find_by_id},
};
use error_stack::{Result, ResultExt};

pub async fn execute(repo_category: &RepoCategory, id: &str) -> Result<(), CategoryError> {
	find_by_id(repo_category, id)
		.await?
		.ok_or(CategoryError::CategoryNotExistError)?;

	repo_category
		.delete(id)
		.await
		.change_context(CategoryError::InternalServerError)?;
	Ok(())
}
