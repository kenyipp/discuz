use actix_web::{test, web, App};
use discuz_core::migration::{Migrator, MigratorTrait};
use discuz_server::app;

#[actix_web::test]
async fn create_category_401() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;

	// It should reject with error code 401
	let req = test::TestRequest::post()
		.uri("/api/category")
		.append_header(("authorization", "bearer INVALID_ACCESS_TOKEN"))
		.to_request();

	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_client_error());
	assert_eq!(resp.status().as_u16(), 400);
}
