use actix_web::{test, web, App};
use discuz_core::migration::{Migrator, MigratorTrait};
use discuz_server::app;

#[actix_web::test]
async fn health_check() {
	let app_state = app::get_app_state().await;
	Migrator::refresh(&app_state.db_connection).await.unwrap();
	let app = test::init_service(
		App::new()
			.app_data(web::Data::new(app_state))
			.service(web::scope("/api").configure(app::get_api_routes)),
	)
	.await;
	let req = test::TestRequest::get()
		.uri("/api/health-check")
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());
	assert_eq!(resp.status().as_u16(), 200);
}
