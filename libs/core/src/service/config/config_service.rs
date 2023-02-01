use error_stack::Result;
use std::fmt::Debug;

pub use crate::{
	repository::repo_config::AppsVersion, service::config::utils::get_config::AppConfig,
};
use crate::{
	repository::repo_config::RepoConfig,
	service::config::{errors::ConfigError, utils},
};

#[derive(Debug, Clone)]
pub struct ConfigService {
	pub repo_config: RepoConfig,
}

#[async_trait]
pub trait ConfigServiceTrait: Sync + Send + Debug {
	async fn get_config(&self) -> Result<AppConfig, ConfigError>;
}

#[async_trait]
impl ConfigServiceTrait for ConfigService {
	async fn get_config(&self) -> Result<AppConfig, ConfigError> {
		utils::get_config::execute(&self.repo_config).await
	}
}
