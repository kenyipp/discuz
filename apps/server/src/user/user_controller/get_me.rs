use serde::Serialize;
use actix_web::HttpResponse;
use crate::{
	auth::errors::AuthError,
	user::dto::user::DtoUser,
	utils::auth::Auth,
	errors::AppError,
};

pub async fn get_me(auth: Auth) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(AuthError::InvalidAccessToken)?;
	Ok(HttpResponse::Ok().json(Response { data: user.into() }))
}

#[derive(Debug, Serialize)]
pub struct Response {
	pub data: DtoUser,
}
