use actix_web::{test, web, App};
use discuz_layers::{
	constants::FAKE_ACCESS_TOKEN,
	migration::{Migrator, MigratorTrait},
	service::{auth::constants::UserRole, user::user_service::UserServiceTrait},
};
use discuz_server::{app, post_category::post_category_controller, user::user_controller};

#[actix_web::test]
async fn create_post_category() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;

	let body = post_category_controller::create::Body {
		name: "New Category Id".to_owned(),
		description: None,
	};

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

	// Run test
	let req = test::TestRequest::post()
		.uri("/api/post/category")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.set_json(body.to_owned())
		.to_request();

	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_success());

	let create_category_resp: post_category_controller::create::Response =
		test::read_body_json(resp).await;

	assert_eq!(create_category_resp.data.name, body.name);
	assert_eq!(create_category_resp.data.description, body.description);
}
