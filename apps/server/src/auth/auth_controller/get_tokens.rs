use actix_web::{web, HttpResponse, Result};
use discuz_layers::service::auth::{
	auth_service::AuthServiceTrait, provider::utils::GetTokensOutput,
};
use serde::{Deserialize, Serialize};

use crate::{auth::errors::ApiAuthError, errors::AppError, utils::app_state::AppState};

pub async fn get_tokens(
	app_state: web::Data<AppState>,
	body: web::Json<Body>,
) -> Result<HttpResponse, AppError> {
	let tokens = app_state
		.auth_service
		.get_tokens(&body.code)
		.await
		.map_err(|error| {
			println!("{:#?}", error);
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
