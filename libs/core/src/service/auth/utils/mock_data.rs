use crate::{
	constants::{FAKE_ACCESS_TOKEN, FAKE_SUB},
	service::auth::utils::AuthUser,
};
use discuz_utils::config::get_config;

pub fn should_return_mock_user_by_access_token(access_token: &str) -> bool {
	let is_development_mode = !get_config().is_production();
	access_token == FAKE_ACCESS_TOKEN && is_development_mode
}

pub fn get_fake_sub() -> String {
	FAKE_SUB.to_owned()
}

pub fn get_mock_auth_user() -> AuthUser {
	AuthUser {
		sub: FAKE_SUB.to_owned(),
		username: FAKE_SUB.to_owned(),
		name: "Fake User".to_owned(),
		email: "fake@user.com".to_owned(),
		picture: None,
		email_verified: None,
		phone_number_verified: None,
	}
}
