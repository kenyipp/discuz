use actix_web::{test, web, App};
use discuz_core::migration::{Migrator, MigratorTrait};
use discuz_server::app;

#[actix_web::test]
async fn get_me_missing_authorization() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;
	let req = test::TestRequest::get().uri("/api/user/me").to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_client_error());
	assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn get_me_invalid_access_token() {
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
		.append_header(("authorization", "basic FAKE_TOKEN".to_string()))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_client_error());
	assert_eq!(resp.status().as_u16(), 401);
}
