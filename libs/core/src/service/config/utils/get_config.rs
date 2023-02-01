use error_stack::{Result, ResultExt};

use crate::{
	repository::repo_config::{AppsVersion, RepoConfig, RepoConfigTrait},
	service::config::errors::ConfigError,
};

pub async fn execute(repo_config: &RepoConfig) -> Result<AppConfig, ConfigError> {
	let versions = repo_config
		.get_apps_versions()
		.await
		.change_context(ConfigError::InternalServerError)?;
	Ok(AppConfig { versions })
}

pub struct AppConfig {
	pub versions: Vec<AppsVersion>,
}
