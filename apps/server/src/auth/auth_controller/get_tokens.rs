use serde::{ Deserialize, Serialize };
use actix_web::{ web, Result, HttpResponse };
use discuz_layers::service::auth::{
	auth_service::AuthServiceTrait,
	provider::utils::GetTokensOutput,
};
use crate::{ errors::AppError, auth::errors::AuthError, utils::app_state::AppState };

pub async fn get_tokens(
	data: web::Data<AppState>,
	body: web::Json<Body>
) -> Result<HttpResponse, AppError> {
	let tokens = data.auth_service.get_tokens(&body.code).await.map_err(|error| {
		println!("{:#?}", error);
		AuthError::InvalidAuthCode
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