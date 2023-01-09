use std::sync::Arc;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use discuz_layers::service::factory::Factory;
use discuz_utils::{amazon::get_aws_sdk_config, config::get_config, get_db_connection};
use dotenv::dotenv;
use futures::join;
use tracing::info;

use crate::{
	auth::auth_route::auth_route, user::user_route::user_route, utils::app_state::AppState,
};

pub async fn listen() -> Result<(), ()> {
	// Get the environment variables
	dotenv().ok();

	// App shared configs
	let config = get_config();
	let db_connection = Arc::new(
		get_db_connection()
			.await
			.expect("Unable to connect the database"),
	);
	let sdk_config = Arc::new(get_aws_sdk_config().await);

	let factory = Factory::new(&db_connection, &sdk_config);

	let auth_service = Arc::new(factory.new_auth_service());
	let user_service = Arc::new(factory.new_user_service(auth_service.clone()));

	// Building the app state which is then pushed to the whole api server
	let app_state = AppState {
		db_connection,
		sdk_config,
		auth_service,
		user_service,
	};

	let port = config.app.port;

	let server = HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(app_state.clone()))
			.wrap(middleware::Logger::default())
			// .route("/api/health-check", web::get().to(health_check))
			.service(web::scope("/api").configure(api_routes))
	})
	.bind(("127.0.0.1", port))
	.unwrap()
	.run();

	let (server_result, ..) = join!(server, async { info!("Server is listened at port {port}") });
	server_result.expect("Unable to start the server");

	Ok(())
}

async fn health_check() -> impl Responder {
	HttpResponse::Ok().body("Ok!")
}

fn api_routes(cfg: &mut web::ServiceConfig) {
	cfg.route("/health-check", web::get().to(health_check));
	cfg.service(web::scope("/auth").configure(auth_route));
	cfg.service(web::scope("/user").configure(user_route));
}
