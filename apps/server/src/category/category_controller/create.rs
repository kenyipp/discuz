use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use discuz_core::service::{
	auth::{auth_service::AuthServiceTrait, constants::UserRole},
	category::category_service::{Category, CategoryServiceTrait, CreateCategoryInput},
};

use crate::{
	auth::errors::ApiAuthError, category::errors::ApiCategoryError, errors::AppError,
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
		sort_index: None,
		user_id: Some(user.id.to_owned()),
		parent_id: body.parent_id.to_owned(),
	};

	let category_service = app_state.category_service.clone();

	let category = category_service
		.create(&input)
		.await
		.map_err(|_| ApiCategoryError::InternalSeverError)?;

	Ok(HttpResponse::Ok().json(Response { data: category }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
	pub name: String,
	pub level: Option<i32>,
	pub postable: Option<bool>,
	pub description: Option<String>,
	pub parent_id: Option<String>,
}
