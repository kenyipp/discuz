use actix_web::{test, web, App};
use discuz_core::{
	constants::FAKE_ACCESS_TOKEN,
	migration::{Migrator, MigratorTrait},
};
use discuz_server::{app, category::category_controller};

#[actix_web::test]
async fn create_category() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;

	let body = category_controller::create::Body {
		name: "New Category Id".to_owned(),
		description: None,
		postable: None,
		level: None,
		parent_id: None,
	};

	// It should reject with error code 403
	let req = test::TestRequest::post()
		.uri("/api/post/category")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.set_json(body.to_owned())
		.to_request();

	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_client_error());
	assert_eq!(resp.status().as_u16(), 403);
}
