use crate::{
	repository::repo_user::{RepoUser, RepoUserTrait},
	service::user::errors::UserError,
};
use error_stack::{Result, ResultExt};

pub async fn execute(repo_user: &RepoUser, user_id: &str) -> Result<(), UserError> {
	repo_user
		.update_user_ban_history_status_to_resolved(user_id)
		.await
		.change_context(UserError::InternalServerError)?;
	Ok(())
}
