use std::sync::Arc;

use aws_config::SdkConfig;
use sea_orm::DatabaseConnection;

use crate::{
	repository::{
		database::{db_file::DbFile, db_user::DbUser},
		repo_file::RepoFile,
		repo_user::RepoUser,
	},
	service::{
		auth::{
			auth_service::{AuthService, AuthServiceTrait},
			provider::api_provider::ApiCognito,
		},
		file::{file_service::FileService, provider::api_provider::ApiS3},
		user::user_service::UserService,
	},
};

use discuz_utils::config::get_config;

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

	pub fn new_file_service(&self) -> FileService {
		let config = get_config();
		let db_file = DbFile::new(&self.db_connection);
		let repo_file = RepoFile::new(db_file);
		let api_provider = Arc::new(ApiS3::new(&self.sdk_config));
		FileService {
			repo_file,
			api_provider,
			bucket: config.amazon.s3.bucket.to_owned(),
		}
	}
}
