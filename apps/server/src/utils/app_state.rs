use std::sync::Arc;

use aws_config::SdkConfig;
use discuz_layers::service::{auth::auth_service::AuthService, user::user_service::UserService};
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
	pub db_connection: Arc<DatabaseConnection>,
	pub sdk_config: Arc<SdkConfig>,
	pub auth_service: Arc<AuthService>,
	pub user_service: Arc<UserService>,
}
