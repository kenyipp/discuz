use actix_web::{test, web, App};
use discuz_layers::{
	constants::FAKE_ACCESS_TOKEN,
	migration::{Migrator, MigratorTrait},
};
use discuz_server::app;
use discuz_server::user::user_controller;

trait BodyTest {
	fn as_str(&self) -> &str;
}

impl BodyTest for web::Bytes {
	fn as_str(&self) -> &str {
		std::str::from_utf8(self).unwrap()
	}
}

#[actix_web::test]
async fn patch_user() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;

	// Step 1: Get the user
	let req = test::TestRequest::get()
		.uri("/api/user/me")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	let get_me_resp: user_controller::get_me::Response = test::read_body_json(resp).await;
	assert!(get_me_resp.data.avatar_url.is_none());

	let new_user_name = "New User Name";
	let new_avatar_url = "http://avatar.com/icon.png";

	let body = user_controller::update_user::Body {
		name: new_user_name.to_owned(),
		avatar_url: Some(new_avatar_url.to_owned()),
	};

	// Step 2: Update user information
	let req = test::TestRequest::patch()
		.uri(&format!("/api/user/{}", get_me_resp.data.id))
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.set_json(body)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	let patch_user_resp: user_controller::update_user::Response = test::read_body_json(resp).await;
	assert!(patch_user_resp.data.avatar_url.is_some());
	assert_eq!(patch_user_resp.data.avatar_url.unwrap(), new_avatar_url);
	assert_eq!(patch_user_resp.data.name, new_user_name);
}
