use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum ApiConfigError {
	InternalSeverError,
}

impl GetErrorDetailTrait for ApiConfigError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			ApiConfigError::InternalSeverError => ErrorDetail {
				code: "config_internal_server_error".to_owned(),
				status: 500,
				message: Some("Internal Server Error".to_owned()),
				detail: None,
			},
		}
	}
}
