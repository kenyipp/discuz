use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use tracing::trace;

use discuz_layers::service::{
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
	if user.status_id != "A" {
		return Err(ApiAuthError::UserBanned.into());
	}

	let auth_service = app_state.auth_service.clone();

	auth_service
		.validate_permission(&user, &[UserRole::Admin])
		.map_err(|_| ApiAuthError::InsufficientPrivilege)?;

	let post_service = app_state.post_service.clone();

	let input = UpdatePostInput {
		id: params.id.to_owned(),
		title: body.title.to_owned(),
		post_category_id: body.post_category_id.to_owned(),
		content: body.content.to_owned(),
		excerpt: body.excerpt.to_owned(),
		user_id: Some(user.id.to_owned()),
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
	pub post_category_id: String,
	pub content: String,
	pub excerpt: Option<String>,
}
