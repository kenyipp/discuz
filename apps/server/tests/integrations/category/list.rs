use actix_web::{test, web, App};
use discuz_core::{
	constants::FAKE_ACCESS_TOKEN,
	migration::{Migrator, MigratorTrait},
	service::{auth::constants::UserRole, user::user_service::UserServiceTrait},
};
use discuz_server::{app, category::category_controller::list::Response, user::user_controller};

#[actix_web::test]
async fn list_category() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;

	// Get the user id
	let req = test::TestRequest::get()
		.uri("/api/user/me")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	let user_body: user_controller::get_me::Response = test::read_body_json(resp).await;
	let user_id = user_body.data.id;

	// Update the user's role
	app_state
		.user_service
		.update_role(&user_id, &UserRole::Admin)
		.await
		.unwrap();

	let req = test::TestRequest::get()
		.uri("/api/category")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	let create_category_resp: Response = test::read_body_json(resp).await;

	assert!(!create_category_resp.data.is_empty());
	assert!(create_category_resp.count > 0);
}
