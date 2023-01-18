use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use tracing::trace;

use discuz_core::service::post::post_service::{Post, PostServiceTrait};

use crate::{errors::AppError, post::errors::ApiPostError, utils::app_state::AppState};

pub async fn execute(
	app_state: web::Data<AppState>,
	params: web::Path<Params>,
) -> Result<HttpResponse, AppError> {
	let post_service = app_state.post_service.clone();

	let post = post_service.find_by_id(params.id).await.map_err(|error| {
		trace!("{:#?}", error);
		ApiPostError::InternalServerError
	})?;

	if let Some(post) = post {
		if post.status_id == *"A" {
			return Ok(HttpResponse::Ok().json(Response { data: post }));
		}
	}

	Ok(HttpResponse::NotFound().finish())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
	pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: Post,
}
