use crate::{ auth::errors::AuthError };
use serde::{ Serialize };
use uuid::Uuid;
use std::{ fmt, convert::From };
use actix_web::{ error::ResponseError, http::StatusCode, HttpResponse };

#[derive(Debug, Serialize, Clone)]
pub struct AppError {
	// Used for tracing the error in log monitor software
	pub id: String,
	//
	pub code: String,
	// Conveying the HTTP status code
	pub status: u16,
	// A short, human-readable message for the general error type; the title should not change for given types
	pub message: Option<String>,
	// A human-readable description of the specific error
	pub detail: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct Response {
	error: AppError,
}

pub struct ErrorDetail {
	//
	pub code: String,
	// Conveying the HTTP status code
	pub status: u16,
	// A short, human-readable message for the general error type; the title should not change for given types
	pub message: Option<String>,
	// A human-readable description of the specific error
	pub detail: Option<String>,
}

pub trait GetErrorDetailTrait {
	fn get_error_detail(&self) -> ErrorDetail;
}

impl AppError {
	pub fn internal_server_error() -> AppError {
		AppError {
			id: Uuid::new_v4().to_string(),
			code: "app_internal_server_error".to_string(),
			status: 500,
			message: Some("Internal Server Error".to_owned()),
			detail: Some(
				"The server encountered an unexpected condition which prevented it from fulfilling the request.".to_owned()
			),
		}
	}
}

impl From<AuthError> for AppError {
	fn from(error: AuthError) -> Self {
		let detail = error.get_error_detail();
		AppError {
			id: Uuid::new_v4().to_string(),
			code: detail.code,
			status: detail.status,
			message: detail.message,
			detail: detail.detail,
		}
	}
}

impl fmt::Display for AppError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:#?}", self)
	}
}

impl ResponseError for AppError {
	fn error_response(&self) -> HttpResponse {
		HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(Response {
			error: self.to_owned(),
		})
	}
}
