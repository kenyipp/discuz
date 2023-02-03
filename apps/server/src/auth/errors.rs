use crate::errors::{ErrorDetail, GetErrorDetailTrait};
use discuz_core::service::auth::errors::AuthError;

#[derive(Debug, Clone)]
pub enum ApiAuthError {
	Generic,
	UserBanned,
	InvalidAuthCode,
	InsufficientPrivilege,
	MissingAuthorization,
	InternalServerError,
}

impl GetErrorDetailTrait for ApiAuthError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			ApiAuthError::Generic => ErrorDetail {
				code: "auth_generic".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			ApiAuthError::InvalidAuthCode => ErrorDetail {
				code: "auth_invalid_auth_token".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			ApiAuthError::InsufficientPrivilege => ErrorDetail {
				code: "auth_insufficient_privilege".to_owned(),
				status: 403,
				message: None,
				detail: None,
			},
			ApiAuthError::UserBanned => ErrorDetail {
				code: "auth_user_banned".to_owned(),
				status: 403,
				message: None,
				detail: None,
			},
			ApiAuthError::MissingAuthorization => ErrorDetail {
				code: "auth_missing_authorization".to_owned(),
				status: 401,
				message: None,
				detail: None,
			},
			ApiAuthError::InternalServerError => ErrorDetail {
				code: "auth_internal_server_error".to_owned(),
				status: 500,
				message: None,
				detail: None,
			},
		}
	}
}

impl From<AuthError> for ApiAuthError {
	fn from(auth_error: AuthError) -> Self {
		match auth_error {
			AuthError::Generic(_) => ApiAuthError::Generic,
			AuthError::InvalidAccessTokenError => ApiAuthError::Generic,
			AuthError::InsufficientPrivilegesError => ApiAuthError::InsufficientPrivilege,
			AuthError::UserBannedError {
				reason: _,
				ban_time: _,
				release_time: _,
			} => ApiAuthError::UserBanned,
			AuthError::InternalServerError => ApiAuthError::InternalServerError,
		}
	}
}
