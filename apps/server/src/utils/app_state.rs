use aws_config::SdkConfig;
use discuz_core::service::prelude::*;
use fred::clients::RedisClient;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
	pub db_connection: Arc<DatabaseConnection>,
	pub sdk_config: Arc<SdkConfig>,
	pub redis_client: Arc<Option<RedisClient>>,
	pub auth_service: Arc<AuthService>,
	pub config_service: Arc<ConfigService>,
	pub file_service: Arc<FileService>,
	pub post_service: Arc<PostService>,
	pub category_service: Arc<CategoryService>,
	pub user_service: Arc<UserService>,
	pub user_ban_history_service: Arc<UserBanHistoryService>,
}
