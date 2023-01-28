pub use crate::repository::{
	database::db_user_ban_history::{
		self, DbUserBanHistory, DbUserBanHistoryTrait, UpdateBanInput, UserBanHistory,
	},
	errors::RepoError,
};
use error_stack::{IntoReport, Result, ResultExt};

#[derive(Debug, Clone)]
pub struct RepoUserBanHistory {
	db_user_ban_history: DbUserBanHistory,
}

impl RepoUserBanHistory {
	pub fn new(db_user_ban_history: DbUserBanHistory) -> RepoUserBanHistory {
		RepoUserBanHistory {
			db_user_ban_history,
		}
	}
}

#[async_trait]
pub trait RepoUserBanHistoryTrait {
	async fn find_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, RepoError>;
	async fn create(&self, input: &CreateBanInput) -> Result<i32, RepoError>;
	async fn update(&self, input: &UpdateBanInput) -> Result<(), RepoError>;
	async fn delete(&self, id: i32) -> Result<(), RepoError>;
	async fn update_status(&self, id: i32, status_id: &str) -> Result<(), RepoError>;
}

#[async_trait]
impl RepoUserBanHistoryTrait for RepoUserBanHistory {
	async fn find_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, RepoError> {
		self.db_user_ban_history
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreateBanInput) -> Result<i32, RepoError> {
		let release_time = input.ban_time.map(|time| {
			let now = chrono::offset::Utc::now();
			now + chrono::Duration::milliseconds(time as i64)
		});

		let db_input = db_user_ban_history::CreateBanInput {
			ban_user_id: input.ban_user_id.to_owned(),
			ban_reason: input.ban_reason.to_owned(),
			ban_time: input.ban_time.to_owned(),
			user_id: input.user_id.to_owned(),
			release_time,
		};

		self.db_user_ban_history
			.create(&db_input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update(&self, input: &UpdateBanInput) -> Result<(), RepoError> {
		self.db_user_ban_history
			.update(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn delete(&self, id: i32) -> Result<(), RepoError> {
		self.db_user_ban_history
			.delete(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update_status(&self, id: i32, status_id: &str) -> Result<(), RepoError> {
		self.db_user_ban_history
			.update_status(id, status_id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}
}

#[derive(Debug, Clone)]
pub struct CreateBanInput {
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: Option<i32>,
	pub user_id: String,
}
