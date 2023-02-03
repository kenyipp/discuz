use actix_web::{test, web, App};
use discuz_core::{
	constants::FAKE_ACCESS_TOKEN,
	migration::{Migrator, MigratorTrait},
	service::prelude::*,
};
use discuz_server::{app, user::user_controller};

#[actix_web::test]
async fn get_me() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;

	let user_service = app_state.user_service.clone();

	let req = test::TestRequest::get()
		.uri("/api/user/me")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	let body: user_controller::get_me::Response = test::read_body_json(resp).await;
	let user_id = body.data.id;

	let input = BanUserInput {
		ban_user_id: user_id.to_owned(),
		ban_reason: None,
		ban_time: None,
		user_id: user_id.to_owned(),
	};

	user_service.ban_user_account(&input).await.unwrap();

	let req = test::TestRequest::get()
		.uri("/api/user/me")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status().as_u16(), 403);
}
