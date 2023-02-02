use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum ApiCategoryError {
	Generic,
	InternalSeverError,
}

impl GetErrorDetailTrait for ApiCategoryError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			ApiCategoryError::Generic => ErrorDetail {
				code: "category_generic".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			ApiCategoryError::InternalSeverError => ErrorDetail {
				code: "post_internal_server_error".to_owned(),
				status: 500,
				message: Some("Internal Server Error".to_owned()),
				detail: None,
			},
		}
	}
}
