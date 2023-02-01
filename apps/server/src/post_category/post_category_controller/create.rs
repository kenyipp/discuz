use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use discuz_core::service::{
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
	let auth_service = app_state.auth_service.clone();
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;

	auth_service
		.validate_user(&user, Some(&[UserRole::Admin]))
		.map_err(|_| ApiAuthError::InsufficientPrivilege)?;

	let input = CreateCategoryInput {
		name: body.name.to_owned(),
		description: body.description.to_owned(),
		postable: body.postable.unwrap_or(false),
		level: body.level.unwrap_or(0),
		user_id: Some(user.id.to_owned()),
		parent_id: body.parent_id.to_owned(),
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
	pub level: Option<i32>,
	pub postable: Option<bool>,
	pub description: Option<String>,
	pub parent_id: Option<String>,
}
