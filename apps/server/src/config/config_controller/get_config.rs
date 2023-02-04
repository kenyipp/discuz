use actix_web::{web, HttpResponse, Result};
use discuz_core::service::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{config::errors::ApiConfigError, errors::AppError, utils::app_state::AppState};

pub async fn execute(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
	let config_service = app_state.config_service.clone();
	let data = config_service
		.get_config()
		.await
		.map_err(|_| ApiConfigError::InternalSeverError)?;
	Ok(HttpResponse::Ok().json(Response { data }))
}

#[derive(Serialize, Deserialize)]
pub struct Response {
	pub data: AppConfig,
}
