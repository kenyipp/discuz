use actix_web::{test, web, App};
use discuz_core::migration::{Migrator, MigratorTrait};
use discuz_server::{app, config::config_controller};

#[actix_web::test]
async fn get_configs() {
	//
	//
	// Step (0): Set up the server
	//
	//
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;

	//
	//
	// Step (1): Sending the request
	//
	//
	let req = test::TestRequest::get().uri("/api/config").to_request();
	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_success());

	let create_post_resp: config_controller::get_config::Response =
		test::read_body_json(resp).await;
	assert_eq!(create_post_resp.data.app_status, "normal");
}
