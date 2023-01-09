use actix_web::HttpResponse;
use serde::Serialize;

use crate::{
	auth::errors::AuthError, errors::AppError, user::dto::user::DtoUser, utils::auth::Auth,
};

pub async fn get_me(auth: Auth) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(AuthError::InvalidAccessToken)?;
	Ok(HttpResponse::Ok().json(Response { data: user.into() }))
}

#[derive(Debug, Serialize)]
pub struct Response {
	pub data: DtoUser,
}
