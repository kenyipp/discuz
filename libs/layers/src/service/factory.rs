use std::sync::Arc;
use aws_config::SdkConfig;
use sea_orm::DatabaseConnection;

/**
 *
 * Auth related modules
 *
 */
use crate::service::auth::auth_service::AuthService;
use crate::service::auth::provider::api_provider::ApiCognito;

/**
 *
 * User related modules
 *
 */
use crate::repository::database::db_user::DbUser;
use crate::repository::repo_user::RepoUser;
use crate::service::user::user_service::UserService;

use super::auth::auth_service::AuthServiceTrait;

pub struct Factory {
	db_connection: Arc<DatabaseConnection>,
	sdk_config: Arc<SdkConfig>,
}

impl Factory {
	pub fn new(db_connection: &Arc<DatabaseConnection>, sdk_config: &Arc<SdkConfig>) -> Factory {
		Factory {
			sdk_config: sdk_config.clone(),
			db_connection: db_connection.clone(),
		}
	}

	pub fn new_auth_service(&self) -> AuthService {
		let api_provider = Arc::new(ApiCognito::new(&self.sdk_config));
		AuthService { api_provider }
	}

	pub fn new_user_service(&self, auth_service: Arc<dyn AuthServiceTrait>) -> UserService {
		let db_user = DbUser::new(&self.db_connection);
		let repo_user = RepoUser::new(db_user);
		UserService {
			repo_user,
			auth_service,
		}
	}
}
