pub use crate::repository::{
	database::db_config::{AppsVersion, Category, Config, DbConfig, DbConfigTrait},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoConfig {
	db_config: DbConfig,
}

impl RepoConfig {
	pub fn new(db_config: DbConfig) -> RepoConfig {
		RepoConfig { db_config }
	}
}

#[async_trait]
pub trait RepoConfigTrait {
	async fn get_apps_versions(&self) -> Result<Vec<AppsVersion>, RepoError>;
	async fn get_categories(&self) -> Result<Vec<Category>, RepoError>;
	async fn get_configs(&self) -> Result<Vec<Config>, RepoError>;
}

#[async_trait]
impl RepoConfigTrait for RepoConfig {
	async fn get_apps_versions(&self) -> Result<Vec<AppsVersion>, RepoError> {
		self.db_config
			.get_apps_versions()
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn get_categories(&self) -> Result<Vec<Category>, RepoError> {
		self.db_config
			.get_categories()
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn get_configs(&self) -> Result<Vec<Config>, RepoError> {
		self.db_config
			.get_configs()
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}
}
