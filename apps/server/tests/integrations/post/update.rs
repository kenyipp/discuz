use actix_web::{test, web, App};
use discuz_layers::{
	constants::{FAKE_ACCESS_TOKEN, UNCLASSIFIED_CATEGORY_ID},
	migration::{Migrator, MigratorTrait},
	service::{auth::constants::UserRole, user::user_service::UserServiceTrait},
	utils::mock_data::{self, post::POST_SECOND_TITLE},
};
use discuz_server::{app, post::post_controller, user::user_controller};

#[actix_web::test]
async fn update_post() {
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
		post_category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
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
	let post_id = create_post_resp.data.id;

	assert_eq!(create_post_resp.data.title, mock_data::post::POST_TITLE);
	assert_eq!(create_post_resp.data.content, mock_data::post::POST_CONTENT);
	assert_eq!(
		create_post_resp.data.post_category_id,
		UNCLASSIFIED_CATEGORY_ID
	);

	//
	//
	// Step (2): Get User and update user role
	//
	//

	let req = test::TestRequest::get()
		.uri("/api/user/me")
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	let user_body: user_controller::get_me::Response = test::read_body_json(resp).await;
	let user_id = user_body.data.id;

	app_state
		.user_service
		.update_role(&user_id, &UserRole::Admin)
		.await
		.unwrap();

	//
	//
	// Step (3): Update the post details
	//
	//

	let body = post_controller::update::Body {
		title: mock_data::post::POST_SECOND_TITLE.to_owned(),
		post_category_id: UNCLASSIFIED_CATEGORY_ID.to_owned(),
		content: mock_data::post::POST_CONTENT.to_owned(),
	};

	let req = test::TestRequest::patch()
		.uri(&format!("/api/post/{post_id}"))
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.set_json(body.to_owned())
		.to_request();

	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_success());

	//
	//
	// Step (4): Get the post detail
	//
	//

	let req = test::TestRequest::get()
		.uri(&format!("/api/post/{post_id}"))
		.append_header(("authorization", "bearer ".to_string() + FAKE_ACCESS_TOKEN))
		.to_request();

	let resp = test::call_service(&app, req).await;

	assert!(resp.status().is_success());

	let get_post_resp: post_controller::get::Response = test::read_body_json(resp).await;

	assert_eq!(get_post_resp.data.title, POST_SECOND_TITLE);
}
