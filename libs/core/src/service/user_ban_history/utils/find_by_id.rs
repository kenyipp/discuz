use crate::{
	repository::repo_user_ban_history::{
		RepoUserBanHistory, RepoUserBanHistoryTrait, UserBanHistory,
	},
	service::user_ban_history::errors::UserBanHistoryError,
};
use error_stack::{FutureExt, Result};

pub async fn execute(
	repo_user_ban_history: &RepoUserBanHistory,
	id: i32,
) -> Result<Option<UserBanHistory>, UserBanHistoryError> {
	let post_reply = repo_user_ban_history
		.find_by_id(id)
		.change_context(UserBanHistoryError::InternalServerError)
		.await?;
	Ok(post_reply)
}
