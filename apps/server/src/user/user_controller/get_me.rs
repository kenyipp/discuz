use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::{
	auth::errors::AuthError, errors::AppError, user::dto::user::DtoUser, utils::auth::Auth,
};

pub async fn execute(auth: Auth) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(AuthError::InvalidAccessToken)?;
	Ok(HttpResponse::Ok().json(Response { data: user.into() }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: DtoUser,
}
