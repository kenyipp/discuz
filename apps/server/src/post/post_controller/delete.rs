use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use tracing::trace;

use discuz_core::service::{
	auth::{auth_service::AuthServiceTrait, constants::UserRole},
	post::{errors::PostError, post_service::PostServiceTrait},
};

use crate::{
	auth::errors::ApiAuthError, errors::AppError, post::errors::ApiPostError,
	utils::app_state::AppState, utils::auth::Auth,
};

pub async fn execute(
	auth: Auth,
	app_state: web::Data<AppState>,
	params: web::Path<Params>,
) -> Result<HttpResponse, AppError> {
	let auth_service = app_state.auth_service.clone();
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;

	auth_service.validate_user(&user, None).map_err(|error| {
		trace!("{:#?}", error);
		ApiAuthError::UserBanned
	})?;

	auth_service
		.validate_user(&user, Some(&[UserRole::Admin]))
		.map_err(|_| ApiAuthError::InsufficientPrivilege)?;

	let post_service = app_state.post_service.clone();

	post_service.delete(params.id).await.map_err(|error| {
		trace!("{:#?}", error);
		match error.current_context() {
			PostError::PostNotExistError => ApiPostError::InvalidRequest {
				detail: Some(format!("Id #{} is not a valid post id", params.id)),
			},
			_ => ApiPostError::InternalServerError,
		}
	})?;

	Ok(HttpResponse::Ok().finish())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
	pub id: i32,
}
