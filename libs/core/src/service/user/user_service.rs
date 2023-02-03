use error_stack::{Result, ResultExt};
use std::{fmt::Debug, sync::Arc};

pub use crate::{
	repository::repo_user::{
		CreateBanHistoryInput, UpdateBanHistoryInput, UpdateUserInput, User, UserBanHistory,
	},
	service::user::utils::get_users::{GetUsersResponse, InputUserList},
};
use crate::{
	repository::repo_user::{RepoUser, RepoUserTrait},
	service::{
		auth::{auth_service::AuthServiceTrait, constants::UserRole},
		user::{errors::UserError, utils},
	},
};

#[derive(Debug, Clone)]
pub struct UserService {
	pub repo_user: RepoUser,
	pub auth_service: Arc<dyn AuthServiceTrait>,
}

#[async_trait]
pub trait UserServiceTrait: Sync + Send + Debug {
	async fn get_profile(&self, access_token: &str) -> Result<User, UserError>;
	async fn get_users(&self, input: Option<&InputUserList>)
		-> Result<GetUsersResponse, UserError>;
	async fn update(&self, input: &UpdateUserInput) -> Result<(), UserError>;
	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), UserError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, UserError>;
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, UserError>;
	//
	async fn find_user_ban_history_by_id(
		&self,
		id: i32,
	) -> Result<Option<UserBanHistory>, UserError>;
	async fn create_user_ban_history(
		&self,
		input: &CreateBanHistoryInput,
	) -> Result<UserBanHistory, UserError>;
	async fn update_user_ban_history(
		&self,
		input: &UpdateBanHistoryInput,
	) -> Result<UserBanHistory, UserError>;
	async fn delete_user_ban_history(&self, user_id: &str) -> Result<(), UserError>;
}

#[async_trait]
impl UserServiceTrait for UserService {
	async fn get_profile(&self, access_token: &str) -> Result<User, UserError> {
		utils::get_profile::execute(&self.repo_user, &*self.auth_service, access_token).await
	}

	async fn get_users(
		&self,
		input: Option<&InputUserList>,
	) -> Result<GetUsersResponse, UserError> {
		utils::get_users::execute(&self.repo_user, input).await
	}

	async fn update(&self, input: &UpdateUserInput) -> Result<(), UserError> {
		self.repo_user
			.update(input)
			.await
			.change_context(UserError::Generic)
	}

	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), UserError> {
		self.repo_user
			.update_role(id, role)
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

	async fn find_user_ban_history_by_id(
		&self,
		id: i32,
	) -> Result<Option<UserBanHistory>, UserError> {
		utils::find_user_ban_history_by_id::execute(&self.repo_user, id).await
	}

	async fn create_user_ban_history(
		&self,
		input: &CreateBanHistoryInput,
	) -> Result<UserBanHistory, UserError> {
		utils::create_user_ban_history::execute(&self.repo_user, input).await
	}

	async fn update_user_ban_history(
		&self,
		input: &UpdateBanHistoryInput,
	) -> Result<UserBanHistory, UserError> {
		utils::update_user_ban_history::execute(&self.repo_user, input).await
	}

	async fn delete_user_ban_history(&self, user_id: &str) -> Result<(), UserError> {
		utils::delete_user_ban_history::execute(&self.repo_user, user_id).await
	}
}
