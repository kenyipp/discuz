pub use crate::repository::repo_user_ban_history::CreateBanInput;
use crate::{
	repository::repo_user_ban_history::{
		RepoUserBanHistory, RepoUserBanHistoryTrait, UserBanHistory,
	},
	service::user_ban_history::{errors::UserBanHistoryError, utils::find_by_id},
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_user_ban_history: &RepoUserBanHistory,
	input: &CreateBanInput,
) -> Result<UserBanHistory, UserBanHistoryError> {
	let user_ban_history_id = repo_user_ban_history
		.create(input)
		.await
		.change_context(UserBanHistoryError::InternalServerError)?;

	let user_ban_history = find_by_id::execute(repo_user_ban_history, user_ban_history_id)
		.await?
		.ok_or(UserBanHistoryError::InternalServerError)?;

	Ok(user_ban_history)
}
