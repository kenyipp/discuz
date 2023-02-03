use sea_orm::{DatabaseConnection, *};
use std::sync::Arc;

use super::entities::{apps_version, category, config};
pub use super::entities::{apps_version::AppsVersion, category::Category, config::Config};

#[derive(Debug, Clone)]
pub struct DbConfig {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbConfigTrait {
	async fn get_apps_versions(&self) -> Result<Vec<AppsVersion>, DbErr>;
	async fn get_categories(&self) -> Result<Vec<Category>, DbErr>;
	async fn get_configs(&self) -> Result<Vec<Config>, DbErr>;
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
	async fn get_configs(&self) -> Result<Vec<Config>, DbErr> {
		config::Entity::find().all(&*self.db_connection).await
	}

	async fn get_apps_versions(&self) -> Result<Vec<AppsVersion>, DbErr> {
		apps_version::Entity::find().all(&*self.db_connection).await
	}

	async fn get_categories(&self) -> Result<Vec<Category>, DbErr> {
		let categories = category::Entity::find()
			.filter(category::Column::StatusId.eq("A"))
			.order_by_asc(category::Column::SortIndex)
			.all(&*self.db_connection)
			.await
			.unwrap();
		Ok(categories)
	}
}
