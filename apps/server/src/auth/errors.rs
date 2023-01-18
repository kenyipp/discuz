use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum ApiAuthError {
	UserBanned,
	InvalidAccessToken,
	InvalidAuthCode,
	InsufficientPrivilege,
	MissingAuthorization,
}

impl GetErrorDetailTrait for ApiAuthError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			ApiAuthError::InvalidAccessToken => ErrorDetail {
				code: "auth_invalid_access_token".to_owned(),
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
		}
	}
}
