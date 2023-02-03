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
	pub amazon: Amazon,
	pub database: Database,
	pub redis: Redis,
}

impl Config {
	pub fn is_production(&self) -> bool {
		!(self.run_mode == "development" || self.run_mode == "ci" || self.run_mode == "testing")
	}
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Redis {
	pub enable: bool,
	pub username: Option<String>,
	pub password: Option<String>,
	pub host: Option<String>,
	pub port: Option<i32>,
	pub database: Option<i32>,
}

impl Redis {
	pub fn get_connection_string(&self) -> String {
		let username = self.username.to_owned().unwrap_or_else(|| "".to_owned());
		let password = self.password.to_owned().unwrap_or_else(|| "".to_owned());
		let database = self.database.unwrap_or(0);
		let host = self
			.host
			.to_owned()
			.unwrap_or_else(|| "localhost".to_owned());
		let port = self.port.unwrap_or(6379);
		if username.is_empty() && password.is_empty() {
			format!("redis://{host}:{port}/{database}")
		} else {
			format!("redis://{username}:{password}@{host}:{port}/{database}")
		}
	}
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
			.merge(Toml::file(format!("config/{run_mode}.toml")))
			.merge(Env::raw().map(|key| {
				let key_string = key.as_str();
				if key_string.starts_with("REDIS_") {
					key_string.replace("REDIS_", "REDIS.").into()
				} else if key_string.starts_with("DATABASE_") {
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
}

#[derive(Debug, Error, Display)]
pub enum ConfigError {
	#[display(fmt = "Generic Error")]
	Generic,
}
