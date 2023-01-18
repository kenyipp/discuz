use std::pin::Pin;

use actix_web::{dev, Error, FromRequest, HttpRequest, Result};
use discuz_core::service::user::user_service::{User, UserServiceTrait};
use futures::Future;
use serde::Deserialize;
use tracing::trace;

use crate::utils::app_state::AppState;

#[derive(Debug, Deserialize)]
pub struct Auth {
	pub user: Option<User>,
}

impl FromRequest for Auth {
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
	fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
		let access_token = Self::extract_access_token(req);
		let user_service = req
			.app_data::<actix_web::web::Data<AppState>>()
			.expect("The user service is not set up in the app state")
			.user_service
			.clone();
		Box::pin(async move {
			if let Some(access_token) = access_token {
				match user_service.get_profile(&access_token).await {
					Ok(user) => Ok(Auth { user: Some(user) }),
					Err(error) => {
						trace!("Unable to create profile by the access token\n{:#?}", error);
						Ok(Auth { user: None })
					}
				}
			} else {
				trace!("Unable to extract access token from the header");
				Ok(Auth { user: None })
			}
		})
	}
}

impl Auth {
	fn extract_access_token(req: &HttpRequest) -> Option<String> {
		if let Some(access_token) = req.headers().get("authorization") {
			let access_token = if let Ok(access_token) = access_token.to_str() {
				access_token.to_owned()
			} else {
				trace!(
					"[Utils - auth.rs] Unable to convert the access token from header value to string"
				);
				return None;
			};
			let mut chucks = access_token.split(' ').to_owned();
			let policy = chucks.next();
			if let Some(policy) = policy {
				if policy.to_lowercase() != "bearer" {
					return None;
				}
			}
			chucks.next().map(|access_token| access_token.to_owned())
		} else {
			None
		}
	}
}
