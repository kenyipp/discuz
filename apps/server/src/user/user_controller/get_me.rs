use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::{
	auth::errors::ApiAuthError, errors::AppError, user::dto::user::DtoUser, utils::auth::Auth,
};

pub async fn execute(auth: Auth) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(ApiAuthError::MissingAuthorization)?;
	Ok(HttpResponse::Ok().json(Response { data: user.into() }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub data: DtoUser,
}
