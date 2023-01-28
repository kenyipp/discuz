use crate::{
	repository::repo_user_ban_history::{RepoUserBanHistory, RepoUserBanHistoryTrait},
	service::user_ban_history::errors::UserBanHistoryError,
};
use error_stack::{FutureExt, Result};

pub async fn execute(
	repo_user_ban_history: &RepoUserBanHistory,
	id: i32,
) -> Result<(), UserBanHistoryError> {
	repo_user_ban_history
		.delete(id)
		.change_context(UserBanHistoryError::InternalServerError)
		.await?;
	Ok(())
}
