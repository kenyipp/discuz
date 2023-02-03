use error_stack::{Result, ResultExt};

use crate::{
	repository::{
		database::db_user::UpdateBanHistoryInput,
		repo_user::{RepoUser, RepoUserTrait, UserBanHistory},
	},
	service::user::{constants::UserStatus, errors::UserError, utils},
};

pub async fn execute(
	repo_user: &RepoUser,
	input: &UpdateBanHistoryInput,
) -> Result<UserBanHistory, UserError> {
	let history = utils::find_user_ban_history_by_id::execute(repo_user, input.id)
		.await?
		.ok_or(UserError::UserBanHistoryNotExistError)?;

	let release_time = input.ban_time.map(|time| {
		let now = chrono::offset::Utc::now();
		now + chrono::Duration::milliseconds(time as i64)
	});

	let repo_input = UpdateBanHistoryInput {
		id: input.id.to_owned(),
		ban_user_id: input.ban_user_id.to_owned(),
		ban_reason: input.ban_reason.to_owned(),
		ban_time: input.ban_time.or(history.ban_time),
		release_time: release_time.or(history.release_time),
		user_id: input.user_id.to_owned(),
	};

	repo_user
		.update_ban_history(&repo_input)
		.await
		.change_context(UserError::InternalServerError)?;

	let history = utils::find_user_ban_history_by_id::execute(repo_user, input.id)
		.await?
		.ok_or(UserError::InternalServerError)?;

	if let Some(release_time) = history.release_time {
		if chrono::offset::Utc::now() > release_time {
			repo_user
				.update_user_status(&history.ban_user_id, &UserStatus::Normal)
				.await
				.change_context(UserError::InternalServerError)?;
		}
	}

	Ok(history)
}
