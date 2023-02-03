#[macro_use]
extern crate tracing;

use dotenv::dotenv;
use std::env;
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
	dotenv().ok();

	// Set the default value of RUST_LOG if it isn't explicitly defined
	if env::var("RUST_LOG").is_err() {
		env::set_var("RUST_LOG", "debug");
	}

	// Set the default value of TRACING_LOG if it isn't explicitly defined
	if env::var("TRACING_LOG").is_err() {
		env::set_var("TRACING_LOG", "discuz-core=debug");
	}

	let subscriber = Subscriber::builder()
		.with_env_filter(EnvFilter::from_default_env())
		.finish();

	// Set up the default logging subscriber for the logging.
	// Allow the logging to print on the screen
	tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

	let result = discuz_server::app::listen().await;
	if let Some(error) = result.err() {
		error!("{:#?}", error);
	}
}
