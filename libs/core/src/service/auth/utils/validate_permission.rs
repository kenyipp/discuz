use std::str::FromStr;

use crate::service::{
	auth::{constants::UserRole, errors::AuthError},
	user::user_service::User,
};

pub fn execute(user: &User, roles: &[UserRole]) -> Result<(), AuthError> {
	let user_role = UserRole::from_str(&user.role).map_err(|_| AuthError::InternalServerError)?;

	if !roles.iter().any(|role| *role == user_role) {
		return Err(AuthError::InsufficientPrivilegesError);
	}

	Ok(())
}
