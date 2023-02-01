use sea_orm::{DatabaseConnection, *};
use std::sync::Arc;

use super::entities::apps_version;
pub use super::entities::apps_version::AppsVersion;

#[derive(Debug, Clone)]
pub struct DbConfig {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbConfigTrait {
	async fn get_apps_versions(&self) -> Result<Vec<AppsVersion>, DbErr>;
}

impl DbConfig {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbConfig {
		DbConfig {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
impl DbConfigTrait for DbConfig {
	async fn get_apps_versions(&self) -> Result<Vec<AppsVersion>, DbErr> {
		apps_version::Entity::find().all(&*self.db_connection).await
	}
}
