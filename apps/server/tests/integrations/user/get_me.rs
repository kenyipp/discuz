use actix_web::{test, web, App};
use discuz_core::{
	constants::FAKE_ACCESS_TOKEN,
	migration::{Migrator, MigratorTrait},
	service::auth::utils::mock_data,
};
use discuz_server::{app, user::user_controller};

trait BodyTest {
	fn as_str(&self) -> &str;
}

impl BodyTest for web::Bytes {
	fn as_str(&self) -> &str {
		std::str::from_utf8(self).unwrap()
	}
}

#[actix_web::test]
async fn get_me() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;
	let req = test::TestRequest::get()
		.uri("/api/user/me")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	let body: user_controller::get_me::Response = test::read_body_json(resp).await;
	let mock_user = mock_data::get_mock_auth_user();
	assert_eq!(body.data.name, mock_user.name);
	assert_eq!(body.data.email, mock_user.email);
	assert!(body.data.avatar_url.is_none());
}
