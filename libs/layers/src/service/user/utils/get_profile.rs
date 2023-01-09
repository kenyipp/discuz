use error_stack::{ Result, ResultExt, Report };
use super::super::{ user_service::User, errors::UserError };
use crate::{
	repository::repo_user::{ CreateUserInput, RepoUser, RepoUserTrait },
	service::auth::auth_service::{ AuthUser, AuthServiceTrait },
};

pub async fn get_profile(
	repo_user: &RepoUser,
	auth_service: &dyn AuthServiceTrait,
	access_token: &str
) -> Result<User, UserError> {
	let sub = auth_service
		.validate_access_token(access_token).await
		.change_context(UserError::InvalidCredentials)
		.attach_printable("Invalid access token")?;

	if let Some(user) = repo_user.find_by_sub(&sub).await.change_context(UserError::Generic)? {
		return Ok(user);
	}

	let auth_user = auth_service
		.get_auth_user_by_access_token(access_token).await
		.change_context(UserError::Generic)?;

	create_profile_by_cognito_user(repo_user, &auth_user).await
}

async fn create_profile_by_cognito_user(
	repo_user: &RepoUser,
	auth_user: &AuthUser
) -> Result<User, UserError> {
	let data = auth_user.clone();

	let input = CreateUserInput {
		name: data.name,
		email: data.email,
		sub: data.sub,
		avatar_url: data.picture,
	};

	let user_id = repo_user.create(&input).await.change_context(UserError::Generic)?;

	if let Some(user) = repo_user.find_by_id(&user_id).await.change_context(UserError::Generic)? {
		return Ok(user);
	}

	Err(Report::new(UserError::Generic))
}
