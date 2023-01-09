use std::{fmt::Debug, sync::Arc};

use error_stack::{Result, ResultExt};

pub use crate::repository::repo_user::{UpdateUserInput, User};
use crate::{
	repository::repo_user::{RepoUser, RepoUserTrait},
	service::{
		auth::auth_service::AuthServiceTrait,
		user::{errors::UserError, utils::get_profile},
	},
};

#[derive(Debug, Clone)]
pub struct UserService {
	pub repo_user: RepoUser,
	pub auth_service: Arc<dyn AuthServiceTrait>,
}

impl UserService {
	pub fn new(repo_user: RepoUser, auth_service: &Arc<dyn AuthServiceTrait>) -> UserService {
		UserService {
			repo_user,
			auth_service: auth_service.clone(),
		}
	}
}

#[async_trait]
pub trait UserServiceTrait: Sync + Send + Debug {
	async fn get_profile(&self, access_token: &str) -> Result<User, UserError>;
	async fn update(&self, input: &UpdateUserInput) -> Result<(), UserError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, UserError>;
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, UserError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
	async fn get_profile(&self, access_token: &str) -> Result<User, UserError> {
		get_profile(&self.repo_user, &*self.auth_service, access_token).await
	}
	async fn update(&self, input: &UpdateUserInput) -> Result<(), UserError> {
		self.repo_user
			.update(input)
			.await
			.change_context(UserError::Generic)
	}
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, UserError> {
		self.repo_user
			.find_by_id(id)
			.await
			.change_context(UserError::Generic)
	}
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, UserError> {
		self.repo_user
			.find_by_sub(sub)
			.await
			.change_context(UserError::Generic)
	}
}
