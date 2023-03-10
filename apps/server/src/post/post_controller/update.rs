use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use tracing::trace;

use discuz_core::service::{
	auth::{auth_service::AuthServiceTrait, constants::UserRole},
	post::{
		errors::PostError,
		post_service::{Post, PostServiceTrait, UpdatePostInput},
	},
};

use crate::{
	auth::errors::ApiAuthError, errors::AppError, post::errors::ApiPostError,
	utils::app_state::AppState, utils::auth::Auth,
};

pub async fn execute(
	app_state: web::Data<AppState>,
	auth: Auth,
	body: web::Json<Body>,
	params: web::Path<Params>,
) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;
	let auth_service = app_state.auth_service.clone();

	auth_service
		.validate_user(&user, Some(&[UserRole::Admin]))
		.map_err(|_| ApiAuthError::InsufficientPrivilege)?;

	let post_service = app_state.post_service.clone();

	let input = UpdatePostInput {
		id: params.id.to_owned(),
		title: body.title.to_owned(),
		category_id: body.category_id.to_owned(),
		max_comment_count: None,
		content: body.content.to_owned(),
		status_id: None,
	};

	let post = post_service.update(&input).await.map_err(|error| {
		trace!("{:#?}", error);
		match error.current_context() {
			PostError::PostNotExistError => ApiPostError::InvalidRequest {
				detail: Some(format!("Id #{} is not a valid post id", params.id)),
			},
			_ => ApiPostError::InternalServerError,
		}
	})?;

	Ok(HttpResponse::Ok().json(Response { data: post }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: Post,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
	pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
	pub title: String,
	pub category_id: String,
	pub content: String,
}
