use actix_web::{web, HttpResponse, Result};
use discuz_core::service::auth::{
	auth_service::AuthServiceTrait, provider::api_provider::GetTokensOutput,
};
use serde::{Deserialize, Serialize};
use tracing::trace;

use crate::{auth::errors::ApiAuthError, errors::AppError, utils::app_state::AppState};

pub async fn execute(
	app_state: web::Data<AppState>,
	body: web::Json<Body>,
) -> Result<HttpResponse, AppError> {
	let tokens = app_state
		.auth_service
		.get_tokens(&body.code)
		.await
		.map_err(|error| {
			trace!("{:#?}", error);
			ApiAuthError::InvalidAuthCode
		})?;
	Ok(HttpResponse::Ok().json(Response { data: tokens }))
}

#[derive(Debug, Serialize)]
pub struct Response {
	pub data: GetTokensOutput,
}

#[derive(Debug, Deserialize)]
pub struct Body {
	pub code: String,
}
