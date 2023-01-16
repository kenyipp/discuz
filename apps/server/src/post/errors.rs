use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum ApiPostError {
	Generic,
	InvalidRequest { detail: Option<String> },
	InternalServerError,
}

impl GetErrorDetailTrait for ApiPostError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			ApiPostError::Generic => ErrorDetail {
				code: "post_generic".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			ApiPostError::InvalidRequest { detail } => ErrorDetail {
				code: "post_invalid_post_request".to_owned(),
				status: 400,
				message: Some("Invalid Request".to_owned()),
				detail: detail.to_owned(),
			},
			ApiPostError::InternalServerError => ErrorDetail {
				code: "post_internal_server_error".to_owned(),
				status: 500,
				message: Some("Internal Server Error".to_owned()),
				detail: None,
			},
		}
	}
}
