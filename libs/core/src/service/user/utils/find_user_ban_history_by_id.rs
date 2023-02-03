use crate::{
	repository::repo_user::{RepoUser, RepoUserTrait, UserBanHistory},
	service::user::errors::UserError,
};
use error_stack::{FutureExt, Result};

pub async fn execute(repo_user: &RepoUser, id: i32) -> Result<Option<UserBanHistory>, UserError> {
	let history = repo_user
		.find_ban_history_by_id(id)
		.change_context(UserError::InternalServerError)
		.await?;
	Ok(history)
}
