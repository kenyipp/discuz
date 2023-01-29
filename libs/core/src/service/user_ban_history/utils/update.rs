use crate::{
	repository::repo_user_ban_history::{
		self, RepoUserBanHistory, RepoUserBanHistoryTrait, UserBanHistory,
	},
	service::{
		user::constants::UserStatus,
		user_ban_history::{errors::UserBanHistoryError, utils::find_by_id},
	},
};
use error_stack::{Result, ResultExt};

pub async fn execute(
	repo_user_ban_history: &RepoUserBanHistory,
	input: &UpdateBanInput,
) -> Result<UserBanHistory, UserBanHistoryError> {
	let history = find_by_id::execute(repo_user_ban_history, input.id)
		.await?
		.ok_or(UserBanHistoryError::UserBanHistoryNotExistError)?;

	let release_time = input.ban_time.map(|time| {
		let now = chrono::offset::Utc::now();
		now + chrono::Duration::milliseconds(time as i64)
	});

	let repo_input = repo_user_ban_history::UpdateBanInput {
		id: input.id.to_owned(),
		ban_user_id: input.ban_user_id.to_owned(),
		ban_reason: input.ban_reason.to_owned(),
		ban_time: input.ban_time.or(history.ban_time),
		release_time: release_time.or(history.release_time),
		user_id: input.user_id.to_owned(),
	};

	repo_user_ban_history
		.update(&repo_input)
		.await
		.change_context(UserBanHistoryError::InternalServerError)?;

	let history = find_by_id::execute(repo_user_ban_history, input.id)
		.await?
		.ok_or(UserBanHistoryError::InternalServerError)?;

	if let Some(release_time) = history.release_time {
		if chrono::offset::Utc::now() > release_time {
			repo_user_ban_history
				.update_user_status(&history.ban_user_id, &UserStatus::Normal)
				.await
				.change_context(UserBanHistoryError::InternalServerError)?;
		}
	}

	Ok(history)
}

#[derive(Debug, Clone)]
pub struct UpdateBanInput {
	pub id: i32,
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: Option<i32>,
	pub user_id: String,
}
