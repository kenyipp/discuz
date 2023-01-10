use crate::service::auth::utils::AuthUser;
use std::env;
pub const FAKE_SUB: &str = "FAKE_SUB";
pub const FAKE_ACCESS_TOKEN: &str = "FAKE_ACCESS_TOKEN";

pub fn should_return_mock_user_by_access_token(access_token: &str) -> bool {
	access_token == FAKE_ACCESS_TOKEN && is_development_mode()
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

fn is_development_mode() -> bool {
	env::var("RUN_MODE").map_or(false, |run_mode| {
		vec!["testing", "development", "ci"].contains(&run_mode.as_str())
	})
}
