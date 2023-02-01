pub use crate::repository::{
	database::db_config::{AppsVersion, DbConfig, DbConfigTrait},
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
}
