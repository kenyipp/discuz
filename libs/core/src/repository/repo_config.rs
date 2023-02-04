use crate::repository::constants;
pub use crate::repository::{
	database::db_config::{AppsVersion, Category, Config, DbConfig, DbConfigTrait},
	errors::RepoError,
};
use discuz_utils::redis::{get_cached_result, set_cached_result};
use error_stack::{IntoReport, Result, ResultExt};
use fred::types::Expiration;

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
		let cache_key = constants::CachingKey::CategoryConfigs.to_string();
		if let Some(categories) = get_cached_result::<Vec<Category>>(&cache_key).await {
			return Ok(categories);
		}
		let categories = self
			.db_config
			.get_categories()
			.await
			.into_report()
			.change_context(RepoError::Generic)?;
		set_cached_result(&cache_key, &categories, Some(Expiration::EX(60))).await;
		Ok(categories)
	}

	async fn get_configs(&self) -> Result<Vec<Config>, RepoError> {
		self.db_config
			.get_configs()
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}
}
