use actix_web::{web, HttpResponse, Result};
use tracing::error;

use discuz_core::service::{auth::constants::UserRole, prelude::*};

use crate::{
	auth::errors::ApiAuthError, category::errors::ApiCategoryError, errors::AppError,
	utils::app_state::AppState, utils::auth::Auth,
};

pub async fn execute(app_state: web::Data<AppState>, auth: Auth) -> Result<HttpResponse, AppError> {
	let auth_service = app_state.auth_service.clone();
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;

	auth_service
		.validate_user(&user, Some(&[UserRole::Admin]))
		.map_err(|_| ApiAuthError::InsufficientPrivilege)?;

	let category_service = app_state.category_service.clone();

	let response = category_service
		.get_categories(None)
		.await
		.map_err(|error| {
			error!("{:#?}", error);
			ApiCategoryError::InternalSeverError
		})?;

	Ok(HttpResponse::Ok().json(response))
}

pub type Response = GetCategoriesResponse;
