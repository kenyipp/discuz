use actix_web::{web, HttpResponse};
use discuz_layers::service::{
	file::file_service::FileServiceTrait,
	file::{constants::FileType, utils::get_upload_url::GetUploadUrlResponse},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::trace;

use crate::{
	auth::errors::ApiAuthError,
	errors::AppError,
	file::errors::ApiFileError,
	utils::{app_state::AppState, auth::Auth},
};

pub async fn execute(
	auth: Auth,
	body: web::Json<Body>,
	app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
	auth.user.ok_or(ApiAuthError::MissingAuthorization)?;
	let file_service = app_state.file_service.clone();
	let file_type =
		FileType::from_str(&body.file_type).map_err(|_| ApiFileError::InvalidFileType {
			detail: Some(format!("Invalid file type {}", body.file_type)),
		})?;
	let response = file_service
		.get_upload_url(&file_type)
		.await
		.map_err(|error| {
			trace!("{:#?}", error);
			AppError::internal_server_error()
		})?;
	Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug, Deserialize)]
pub struct Body {
	#[serde(rename = "type")]
	pub file_type: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
	data: GetUploadUrlResponse,
}
