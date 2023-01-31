use error_stack::{Result, ResultExt};
use futures::try_join;

pub use crate::repository::repo_user::InputUserList;

use crate::{
	repository::repo_user::{RepoUser, RepoUserTrait},
	service::user::{errors::UserError, user_service::User},
};

pub async fn execute(
	repo_user: &RepoUser,
	input: Option<&InputUserList>,
) -> Result<GetUsersResponse, UserError> {
	let default_input = InputUserList::default();
	let input = input.unwrap_or(&default_input);
	let (data, count) = try_join!(repo_user.list(&input), repo_user.count(&input.filter))
		.change_context(UserError::InternalServerError)?;
	Ok(GetUsersResponse { data, count })
}

#[derive(Debug, Clone)]
pub struct GetUsersResponse {
	pub data: Vec<User>,
	pub count: u64,
}
