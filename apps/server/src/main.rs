#[macro_use]
extern crate tracing;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
	dotenv().ok();

	// Set the default value of RUST_LOG if it isn't explicitly defined
	if env::var("RUST_LOG").is_err() {
		env::set_var("RUST_LOG", "discuz_server=debug,actix_web=info");
	}
	// Set up the default logging subscriber for the logging. Allow the logging to print on the screen
	tracing_subscriber::fmt().init();

	let result = discuz_server::app::listen().await;
	if let Some(error) = result.err() {
		error!("{:#?}", error);
	}
}
