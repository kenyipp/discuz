use crate::{
	repository::repo_user::{CreateBanHistoryInput, RepoUser, RepoUserTrait, UserBanHistory},
	service::user::{constants::UserStatus, errors::UserError, utils},
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_user: &RepoUser,
	input: &CreateBanHistoryInput,
) -> Result<UserBanHistory, UserError> {
	let user_ban_history_id = repo_user
		.create_ban_history(input)
		.await
		.change_context(UserError::InternalServerError)?;

	repo_user
		.update_user_status(&input.ban_user_id.to_owned(), &UserStatus::Banned)
		.await
		.change_context(UserError::InternalServerError)?;

	let user_ban_history =
		utils::find_user_ban_history_by_id::execute(repo_user, user_ban_history_id)
			.await?
			.ok_or(UserError::InternalServerError)?;

	Ok(user_ban_history)
}
