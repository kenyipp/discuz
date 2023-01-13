use std::{env, sync::Arc};

use derive_more::{Display, Error};
use dotenv::dotenv;
use error_stack::{IntoReport, Result, ResultExt};
use figment::{
	providers::{Env, Format, Toml},
	Figment,
};
use serde::Serialize;
use serde_derive::Deserialize;

lazy_static! {
	static ref CONFIG: Arc<Config> = Arc::new(Config::new().unwrap());
}

// Function to retrieve the static config object
// It lives for the entire lifetime of the running program
pub fn get_config() -> &'static Config {
	&CONFIG
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
	pub run_mode: String,
	pub app: App,
	pub database: Database,
	pub amazon: Amazon,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
	pub port: u16,
	pub allowed_origin: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
	pub hostname: String,
	pub username: String,
	pub password: String,
	pub port: i32,
	pub database: String,
	// We will override the database connection if the URL property is specified
	pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Amazon {
	pub region: String,
	pub cognito: Cognito,
	pub s3: S3,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cognito {
	pub user_pool_id: String,
	pub domain: String,
	pub client_id: String,
	pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3 {
	pub bucket: String,
}

impl Config {
	pub fn new() -> Result<Self, ConfigError> {
		dotenv().ok();
		let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());
		Figment::new()
			.merge(Toml::file("config/default.toml"))
			.merge(Toml::file(format!("config/{}.toml", run_mode)))
			.merge(Env::raw().map(|key| {
				let key_string = key.as_str();
				if key_string.starts_with("DATABASE_") {
					key_string.replace("DATABASE_", "DATABASE.").into()
				} else if key_string.starts_with("AWS_COGNITO_") {
					key_string.replace("AWS_COGNITO_", "AMAZON.COGNITO.").into()
				} else if key_string.starts_with("AWS_S3_") {
					key_string.replace("AWS_S3_", "AMAZON.S3.").into()
				} else if key_string.starts_with("AWS_") {
					key_string.replace("AWS_", "AMAZON.").into()
				} else {
					key_string.into()
				}
			}))
			.extract()
			.into_report()
			.change_context(ConfigError::Generic)
	}

	pub fn is_development(&self) -> bool {
		self.run_mode == "development"
	}
}

#[derive(Debug, Error, Display)]
pub enum ConfigError {
	#[display(fmt = "Generic Error")]
	Generic,
}
