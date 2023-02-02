use actix_web::{test, web, App};
use discuz_core::{
	constants::{FAKE_ACCESS_TOKEN, UNCLASSIFIED_CATEGORY_ID},
	migration::{Migrator, MigratorTrait},
	utils::mock_data,
};
use discuz_server::{app, post::post_controller};

#[actix_web::test]
async fn create_post() {
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
	// Step (1): Create post
	//
	//

	let body = post_controller::create::Body {
		title: mock_data::post::POST_TITLE.to_owned(),
		category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: mock_data::post::POST_CONTENT.to_owned(),
	};

	let req = test::TestRequest::post()
		.uri("/api/post")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.set_json(body.to_owned())
		.to_request();

	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_success());

	let create_post_resp: post_controller::create::Response = test::read_body_json(resp).await;

	assert_eq!(create_post_resp.data.title, mock_data::post::POST_TITLE);
	assert_eq!(create_post_resp.data.content, mock_data::post::POST_CONTENT);
	assert_eq!(create_post_resp.data.category_id, UNCLASSIFIED_CATEGORY_ID);
}
