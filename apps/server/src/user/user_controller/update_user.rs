use serde::{ Serialize, Deserialize };
use actix_web::{ web, HttpResponse };
use tracing::trace;
use discuz_layers::service::user::user_service::{ UpdateUserInput, UserServiceTrait };
use crate::{
	errors::AppError,
	user::dto::user::DtoUser,
	utils::auth::Auth,
	utils::app_state::AppState,
	auth::errors::AuthError,
};

pub async fn update_user(
	params: web::Path<Params>,
	body: web::Json<Body>,
	app_state: web::Data<AppState>,
	auth: Auth
) -> Result<HttpResponse, AppError> {
	let user = auth.user.ok_or(AuthError::InvalidAccessToken)?;
	let id = params.id.to_owned();

	if user.id != id {
		return Err(AuthError::InsufficientPrivilege.into());
	}

	let updates = UpdateUserInput {
		id: id.to_owned(),
		name: body.name.to_owned(),
		avatar_url: body.avatar_url.to_owned(),
	};

	let user_service = app_state.user_service.clone();

	user_service.update(&updates).await.map_err(|error| {
		trace!("{:#?}", error);
		AppError::internal_server_error()
	})?;

	let user = user_service
		.find_by_id(&id).await
		.map_err(|error| {
			trace!("{:#?}", error);
			AppError::internal_server_error()
		})?
		.ok_or_else(AppError::internal_server_error)?;

	Ok(HttpResponse::Ok().json(Response { data: user.into() }))
}

#[derive(Debug, Deserialize)]
pub struct Params {
	pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Body {
	pub name: String,
	pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
	pub data: DtoUser,
}
