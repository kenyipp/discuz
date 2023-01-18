use std::sync::Arc;

use aws_config::SdkConfig;
use sea_orm::DatabaseConnection;

use crate::{
	repository::{
		database::{
			db_file::DbFile, db_post::DbPost, db_post_category::DbPostCategory,
			db_post_comment::DbPostComment, db_user::DbUser,
		},
		repo_file::RepoFile,
		repo_post::RepoPost,
		repo_post_category::RepoPostCategory,
		repo_post_comment::RepoPostComment,
		repo_user::RepoUser,
	},
	service::{
		auth::{
			auth_service::{AuthService, AuthServiceTrait},
			provider::api_provider::ApiCognito,
		},
		file::{file_service::FileService, provider::api_provider::ApiS3},
		post::post_service::PostService,
		post_category::post_category_service::PostCategoryService,
		post_comment::post_comment_service::PostCommentService,
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

	pub fn new_post_service(&self) -> PostService {
		let db_post = DbPost::new(&self.db_connection);
		let repo_post = RepoPost::new(db_post);
		PostService { repo_post }
	}

	pub fn new_post_comment_service(&self) -> PostCommentService {
		let db_post_comment = DbPostComment::new(&self.db_connection);
		let repo_post_comment = RepoPostComment::new(db_post_comment);
		PostCommentService { repo_post_comment }
	}

	pub fn new_post_category_service(&self) -> PostCategoryService {
		let db_post_category = DbPostCategory::new(&self.db_connection);
		let repo_post_category = RepoPostCategory::new(db_post_category);
		PostCategoryService { repo_post_category }
	}

	pub fn new_user_service(&self, auth_service: Arc<dyn AuthServiceTrait>) -> UserService {
		let db_user = DbUser::new(&self.db_connection);
		let repo_user = RepoUser::new(db_user);
		UserService {
			repo_user,
			auth_service,
		}
	}

	pub fn new_auth_service(&self) -> AuthService {
		let api_provider = Arc::new(ApiCognito::new(&self.sdk_config));
		AuthService { api_provider }
	}
}
