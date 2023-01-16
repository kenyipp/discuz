use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum ApiPostCategoryError {
	Generic,
	InternalSeverError,
}

impl GetErrorDetailTrait for ApiPostCategoryError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			ApiPostCategoryError::Generic => ErrorDetail {
				code: "post_category_generic".to_owned(),
				status: 400,
				message: None,
				detail: None,
			},
			ApiPostCategoryError::InternalSeverError => ErrorDetail {
				code: "post_internal_server_error".to_owned(),
				status: 500,
				message: Some("Internal Server Error".to_owned()),
				detail: None,
			},
		}
	}
}
