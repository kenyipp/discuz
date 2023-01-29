use std::str::FromStr;

use crate::service::{
	auth::{constants::UserRole, errors::AuthError, utils::validate_permission},
	user::{constants::UserStatus, user_service::User},
};

pub fn execute(user: &User, roles: Option<&[UserRole]>) -> Result<(), AuthError> {
	if let Some(roles) = roles {
		validate_permission::execute(user, roles)?;
	}

	let user_status = UserStatus::from_str(&user.status_id.to_owned())
		.map_err(|_| AuthError::InternalServerError)?;

	match user_status {
		UserStatus::Normal => Ok(()),
		// TODO: fetch the ban data from database
		UserStatus::Banned => Err(AuthError::UserBannedError {
			reason: Some(
				"Your account has been banned by the admin because of violating our terms of service"
					.to_owned(),
			),
			ban_time: None,
			release_time: None,
		}),
		UserStatus::Deactivated => Err(AuthError::UserBannedError {
			reason: Some(
				"Your account has been deleted by the admin for violating our terms of service"
					.to_owned(),
			),
			ban_time: None,
			release_time: None,
		}),
	}
}
