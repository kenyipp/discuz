use aws_config::SdkConfig;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use discuz_layers::service::{
	auth::auth_service::AuthService, file::file_service::FileService,
	post::post_service::PostService, post_category::post_category_service::PostCategoryService,
	user::user_service::UserService,
};

#[derive(Debug, Clone)]
pub struct AppState {
	pub db_connection: Arc<DatabaseConnection>,
	pub sdk_config: Arc<SdkConfig>,
	pub auth_service: Arc<AuthService>,
	pub file_service: Arc<FileService>,
	pub post_service: Arc<PostService>,
	pub post_category_service: Arc<PostCategoryService>,
	pub user_service: Arc<UserService>,
}
