use crate::errors::{ErrorDetail, GetErrorDetailTrait};

#[derive(Debug, Clone)]
pub enum FileError {
	InvalidFileType { detail: Option<String> },
}

impl GetErrorDetailTrait for FileError {
	fn get_error_detail(&self) -> ErrorDetail {
		match self {
			FileError::InvalidFileType { detail } => ErrorDetail {
				code: "file_invalid_file_type".to_owned(),
				status: 400,
				message: Some("Invalid File Type".to_owned()),
				detail: detail.to_owned(),
			},
		}
	}
}
