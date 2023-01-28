use error_stack::Result;
use std::fmt::Debug;

pub use crate::{
	repository::repo_user_ban_history::{CreateBanInput, UserBanHistory},
	service::user_ban_history::utils::update::UpdateBanInput,
};

use crate::{
	repository::repo_user_ban_history::RepoUserBanHistory,
	service::user_ban_history::{errors::UserBanHistoryError, utils},
};

#[derive(Debug, Clone)]
pub struct UserBanHistoryService {
	pub repo_user_ban_history: RepoUserBanHistory,
}

#[async_trait]
pub trait UserBanHistoryServiceTrait: Sync + Send + Debug {
	async fn find_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, UserBanHistoryError>;
	async fn create(&self, input: &CreateBanInput) -> Result<UserBanHistory, UserBanHistoryError>;
	async fn update(&self, input: &UpdateBanInput) -> Result<UserBanHistory, UserBanHistoryError>;
	async fn delete(&self, id: i32) -> Result<(), UserBanHistoryError>;
}

#[async_trait]
impl UserBanHistoryServiceTrait for UserBanHistoryService {
	async fn find_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, UserBanHistoryError> {
		utils::find_by_id::execute(&self.repo_user_ban_history, id).await
	}

	async fn create(&self, input: &CreateBanInput) -> Result<UserBanHistory, UserBanHistoryError> {
		utils::create::execute(&self.repo_user_ban_history, input).await
	}

	async fn update(&self, input: &UpdateBanInput) -> Result<UserBanHistory, UserBanHistoryError> {
		utils::update::execute(&self.repo_user_ban_history, input).await
	}

	async fn delete(&self, id: i32) -> Result<(), UserBanHistoryError> {
		utils::delete::execute(&self.repo_user_ban_history, id).await
	}
}
