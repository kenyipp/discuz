use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum AuthError {
	InvalidUserInput { detail: Option<String> },
	UnexpectedError { detail: Option<String> },
	InvalidAccessToken,
	InvalidAuthCode,
	InsufficientPrivilege,
	MissingAuthorization,
}

impl GetErrorDetailTrait for AuthError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			AuthError::InvalidUserInput { detail } => ErrorDetail {
				code: "auth_invalid_user_input".to_owned(),
				status: 400,
				message: None,
				detail: detail.to_owned(),
			},
			AuthError::InvalidAccessToken => ErrorDetail {
				code: "auth_invalid_access_token".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			AuthError::UnexpectedError { detail } => ErrorDetail {
				code: "auth_unexpected_error".to_owned(),
				status: 500,
				message: None,
				detail: detail.to_owned(),
			},
			AuthError::InvalidAuthCode => ErrorDetail {
				code: "auth_invalid_auth_token".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			AuthError::InsufficientPrivilege => ErrorDetail {
				code: "auth_insufficient_privilege".to_owned(),
				status: 403,
				message: None,
				detail: None,
			},
			AuthError::MissingAuthorization => ErrorDetail {
				code: "auth_missing_authorization".to_owned(),
				status: 401,
				message: None,
				detail: None,
			},
		}
	}
}
