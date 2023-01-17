use actix_cors::Cors;
use actix_web::{
	http::header::{ACCEPT, AUTHORIZATION},
	middleware, web, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use futures::join;
use std::sync::Arc;
use tracing::info;

use discuz_layers::service::factory::Factory;
use discuz_utils::{amazon::get_aws_sdk_config, config::get_config, get_db_connection, redis};

use crate::{
	auth::auth_route, file::file_route, post::post_route, post_category::post_category_route,
	user::user_route, utils::app_state::AppState,
};

pub async fn listen() -> std::io::Result<()> {
	// Get the environment variables
	dotenv().ok();

	// App shared configs
	let config = get_config();
	let app_state = get_app_state().await;

	let port = config.app.port;

	let server = HttpServer::new(move || {
		let cors = get_cors_middleware();
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.wrap(cors)
			.wrap(middleware::Logger::new("%a - %r - %s - %Dms"))
			// .route("/api/health-check", web::get().to(health_check))
			.service(web::scope("/api").configure(get_api_routes))
	})
	.bind(("127.0.0.1", port))
	.unwrap_or_else(|_| panic!("Could not bind server to port #{port}"))
	.run();

	let (server_result, ..) = join!(server, async { info!("Server is listened at port {port}") });
	server_result?;

	Ok(())
}

pub async fn get_app_state() -> AppState {
	// Get the environment variables
	dotenv().ok();

	// App shared configs
	let db_connection = Arc::new(
		get_db_connection()
			.await
			.expect("Unable to connect the database"),
	);
	let sdk_config = Arc::new(get_aws_sdk_config().await);

	// Build the services which share
	let factory = Factory::new(&db_connection, &sdk_config);

	let auth_service = Arc::new(factory.new_auth_service());
	let user_service = Arc::new(factory.new_user_service(auth_service.clone()));
	let file_service = Arc::new(factory.new_file_service());
	let post_service = Arc::new(factory.new_post_service());
	let post_category_service = Arc::new(factory.new_post_category_service());
	let redis_client = Arc::new(
		redis::get_redis_connection()
			.await
			.expect("Unable to connect the redis server"),
	);

	// Building the app state which is then pushed to the whole api server
	AppState {
		db_connection,
		sdk_config,
		auth_service,
		user_service,
		file_service,
		post_service,
		post_category_service,
		redis_client,
	}
}

pub fn get_cors_middleware() -> Cors {
	let config = get_config();
	match config.app.allowed_origin {
		Some(ref origin) => Cors::default()
			.allowed_origin(origin)
			.allowed_headers(vec![AUTHORIZATION, ACCEPT])
			.max_age(3600),
		None => Cors::default()
			.allow_any_origin()
			.send_wildcard()
			.allowed_headers(vec![AUTHORIZATION, ACCEPT])
			.max_age(3600),
	}
}

async fn health_check() -> impl Responder {
	HttpResponse::Ok().body("Ok!")
}

pub fn get_api_routes(cfg: &mut web::ServiceConfig) {
	cfg.route("/health-check", web::get().to(health_check));
	cfg.service(web::scope("/auth").configure(auth_route::route));
	cfg.service(web::scope("/file").configure(file_route::route));
	cfg.service(web::scope("/user").configure(user_route::route));
	cfg.service(web::scope("/post/category").configure(post_category_route::route));
	cfg.service(web::scope("/post").configure(post_route::route));
}
