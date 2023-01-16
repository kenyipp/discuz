use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use tracing::trace;

use discuz_layers::service::post::post_service::{CreatePostInput, Post, PostServiceTrait};

use crate::{
	auth::errors::ApiAuthError, errors::AppError, post::errors::ApiPostError,
	utils::app_state::AppState, utils::auth::Auth,
};

pub async fn execute(
	app_state: web::Data<AppState>,
	auth: Auth,
	body: web::Json<Body>,
) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;
	if user.status_id != "A" {
		return Err(ApiAuthError::UserBanned.into());
	}

	let post_service = app_state.post_service.clone();

	let input = CreatePostInput {
		title: body.title.to_owned(),
		post_category_id: body.post_category_id.to_owned(),
		content: body.content.to_owned(),
		excerpt: body.excerpt.to_owned(),
		user_id: Some(user.id),
	};

	let post = post_service.create(&input).await.map_err(|error| {
		trace!("{:#?}", error);
		ApiPostError::InternalServerError
	})?;

	Ok(HttpResponse::Ok().json(Response { data: post }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: Post,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
	pub title: String,
	pub post_category_id: String,
	pub content: String,
	pub excerpt: Option<String>,
}
