use aws_config::SdkConfig;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::{
	repository::{
		database::{
			category::DbCategory, db_config::DbConfig, db_file::DbFile, db_post::DbPost,
			db_user::DbUser, db_user_ban_history::DbUserBanHistory,
		},
		repo_category::RepoCategory,
		repo_config::RepoConfig,
		repo_file::RepoFile,
		repo_post::RepoPost,
		repo_user::RepoUser,
		repo_user_ban_history::RepoUserBanHistory,
	},
	service::{
		auth::{
			auth_service::{AuthService, AuthServiceTrait},
			provider::api_provider::ApiCognito,
		},
		category::category_service::CategoryService,
		config::config_service::ConfigService,
		file::{file_service::FileService, provider::api_provider::ApiS3},
		post::post_service::PostService,
		user::user_service::UserService,
		user_ban_history::user_ban_history_service::UserBanHistoryService,
	},
};

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

	pub fn new_config_service(&self) -> ConfigService {
		let db_config = DbConfig::new(&self.db_connection);
		let repo_config = RepoConfig::new(db_config);
		ConfigService { repo_config }
	}

	pub fn new_file_service(&self) -> FileService {
		let db_file = DbFile::new(&self.db_connection);
		let repo_file = RepoFile::new(db_file);
		let api_provider = Arc::new(ApiS3::new(&self.sdk_config));
		FileService {
			repo_file,
			api_provider,
		}
	}

	pub fn new_post_service(&self) -> PostService {
		let db_post = DbPost::new(&self.db_connection);
		let repo_post = RepoPost::new(db_post);
		PostService { repo_post }
	}

	pub fn new_category_service(&self) -> CategoryService {
		let category = DbCategory::new(&self.db_connection);
		let repo_category = RepoCategory::new(category);
		CategoryService { repo_category }
	}

	pub fn new_user_service(&self, auth_service: Arc<dyn AuthServiceTrait>) -> UserService {
		let db_user = DbUser::new(&self.db_connection);
		let repo_user = RepoUser::new(db_user);
		UserService {
			repo_user,
			auth_service,
		}
	}

	pub fn new_user_ban_history_service(&self) -> UserBanHistoryService {
		let db_user_ban_history = DbUserBanHistory::new(&self.db_connection);
		let repo_user_ban_history = RepoUserBanHistory::new(db_user_ban_history);
		UserBanHistoryService {
			repo_user_ban_history,
		}
	}
}
