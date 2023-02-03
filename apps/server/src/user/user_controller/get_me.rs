use actix_web::{web, HttpResponse, Result};
use discuz_core::service::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
	auth::errors::ApiAuthError, errors::AppError, user::dto::user::DtoUser,
	utils::app_state::AppState, utils::auth::Auth,
};

pub async fn execute(app_state: web::Data<AppState>, auth: Auth) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;

	let auth_service = app_state.auth_service.clone();
	auth_service.validate_user(&user, None).map_err(|error| {
		let error = error.current_context().to_owned();
		ApiAuthError::from(error)
	})?;

	Ok(HttpResponse::Ok().json(Response { data: user.into() }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: DtoUser,
}
