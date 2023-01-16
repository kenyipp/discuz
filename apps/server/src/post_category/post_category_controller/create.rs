use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use discuz_layers::service::{
	auth::{auth_service::AuthServiceTrait, constants::UserRole},
	post_category::post_category_service::{
		CreateCategoryInput, DefPostCategory, PostCategoryServiceTrait,
	},
};

use crate::{
	auth::errors::ApiAuthError, errors::AppError, post_category::errors::ApiPostCategoryError,
	utils::app_state::AppState, utils::auth::Auth,
};

pub async fn execute(
	app_state: web::Data<AppState>,
	body: web::Json<Body>,
	auth: Auth,
) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;

	let auth_service = app_state.auth_service.clone();

	auth_service
		.validate_permission(&user, &[UserRole::Admin])
		.map_err(|_| ApiAuthError::InsufficientPrivilege)?;

	let input = CreateCategoryInput {
		name: body.name.to_owned(),
		description: body.description.to_owned(),
		user_id: Some(user.id.to_owned()),
	};

	let post_category_service = app_state.post_category_service.clone();

	let post_category = post_category_service
		.create(&input)
		.await
		.map_err(|_| ApiPostCategoryError::InternalSeverError)?;

	Ok(HttpResponse::Ok().json(Response {
		data: post_category,
	}))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: DefPostCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
	pub name: String,
	pub description: Option<String>,
}
