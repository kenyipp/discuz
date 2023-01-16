use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum ApiAuthError {
	InvalidUserInput { detail: Option<String> },
	UnexpectedError { detail: Option<String> },
	UserBanned,
	InvalidAccessToken,
	InvalidAuthCode,
	InsufficientPrivilege,
	MissingAuthorization,
}

impl GetErrorDetailTrait for ApiAuthError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			ApiAuthError::InvalidUserInput { detail } => ErrorDetail {
				code: "auth_invalid_user_input".to_owned(),
				status: 400,
				message: None,
				detail: detail.to_owned(),
			},
			ApiAuthError::InvalidAccessToken => ErrorDetail {
				code: "auth_invalid_access_token".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			ApiAuthError::UnexpectedError { detail } => ErrorDetail {
				code: "auth_unexpected_error".to_owned(),
				status: 500,
				message: None,
				detail: detail.to_owned(),
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
