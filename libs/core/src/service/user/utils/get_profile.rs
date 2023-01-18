use error_stack::{Report, Result, ResultExt};

use crate::{
	constants::{FAKE_ACCESS_TOKEN, FAKE_SUB},
	repository::repo_user::{CreateUserInput, RepoUser, RepoUserTrait},
	service::{
		auth::auth_service::{AuthServiceTrait, AuthUser},
		user::{errors::UserError, user_service::User},
	},
};

use discuz_utils::config::get_config;

pub async fn get_profile(
	repo_user: &RepoUser,
	auth_service: &dyn AuthServiceTrait,
	access_token: &str,
) -> Result<User, UserError> {
	let sub = if !get_config().is_production() && access_token == FAKE_ACCESS_TOKEN {
		FAKE_SUB.to_owned()
	} else {
		auth_service
			.validate_access_token(access_token)
			.await
			.change_context(UserError::InvalidCredentials)
			.attach_printable("Invalid access token")?
	};

	if let Some(user) = repo_user
		.find_by_sub(&sub)
		.await
		.change_context(UserError::Generic)?
	{
		return Ok(user);
	}

	let auth_user = auth_service
		.get_auth_user_by_access_token(access_token)
		.await
		.change_context(UserError::Generic)?;

	create_profile_by_cognito_user(repo_user, &auth_user).await
}

async fn create_profile_by_cognito_user(
	repo_user: &RepoUser,
	auth_user: &AuthUser,
) -> Result<User, UserError> {
	let data = auth_user.clone();

	let input = CreateUserInput {
		name: data.name,
		email: data.email,
		sub: data.sub,
		avatar_url: data.picture,
	};

	let user_id = repo_user
		.create(&input)
		.await
		.change_context(UserError::Generic)?;

	if let Some(user) = repo_user
		.find_by_id(&user_id)
		.await
		.change_context(UserError::Generic)?
	{
		return Ok(user);
	}

	Err(Report::new(UserError::Generic))
}
