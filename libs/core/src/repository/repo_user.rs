use error_stack::{IntoReport, Result, ResultExt};

pub use crate::repository::{
	database::db_user::{
		CreateUserInput, DbUser, DbUserTrait, InputUserList, UpdateBanHistoryInput,
		UpdateUserInput, User, UserBanHistory, UserFilter,
	},
	errors::RepoError,
};
use crate::service::{auth::constants::UserRole, user::constants::UserStatus};

use super::database::db_user;

#[derive(Debug, Clone)]
pub struct RepoUser {
	db_user: DbUser,
}

impl RepoUser {
	pub fn new(db_user: DbUser) -> RepoUser {
		RepoUser { db_user }
	}
}

#[async_trait]
pub trait RepoUserTrait {
	// Basic actions
	//  -> Queries
	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, RepoError>;
	async fn count(&self, filter: &UserFilter) -> Result<u64, RepoError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepoError>;
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, RepoError>;
	//  -> Mutations
	async fn create(&self, input: &CreateUserInput) -> Result<String, RepoError>;
	async fn update(&self, input: &UpdateUserInput) -> Result<(), RepoError>;
	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), RepoError>;
	async fn update_user_status(&self, id: &str, status_id: &UserStatus) -> Result<(), RepoError>;

	// Ban user
	// -> Queries
	async fn find_ban_history_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, RepoError>;
	// -> Mutations
	async fn create_ban_history(&self, input: &CreateBanHistoryInput) -> Result<i32, RepoError>;
	async fn update_ban_history(&self, input: &UpdateBanHistoryInput) -> Result<(), RepoError>;
	async fn update_user_ban_history_status_to_resolved(
		&self,
		user_id: &str,
	) -> Result<(), RepoError>;
}

#[async_trait]
impl RepoUserTrait for RepoUser {
	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, RepoError> {
		self.db_user
			.list(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn count(&self, filter: &UserFilter) -> Result<u64, RepoError> {
		self.db_user
			.count(filter)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepoError> {
		self.db_user
			.find_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, RepoError> {
		self.db_user
			.find_by_sub(sub)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create(&self, input: &CreateUserInput) -> Result<String, RepoError> {
		self.db_user
			.create(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update(&self, input: &UpdateUserInput) -> Result<(), RepoError> {
		self.db_user
			.update(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), RepoError> {
		self.db_user
			.update_role(id, role)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update_user_status(&self, id: &str, status_id: &UserStatus) -> Result<(), RepoError> {
		self.db_user
			.update_user_status(id, status_id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn find_ban_history_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, RepoError> {
		self.db_user
			.find_ban_history_by_id(id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn create_ban_history(&self, input: &CreateBanHistoryInput) -> Result<i32, RepoError> {
		let release_time = input.ban_time.map(|time| {
			let now = chrono::offset::Utc::now();
			now + chrono::Duration::milliseconds(time as i64)
		});

		let db_input = db_user::CreateBanHistoryInput {
			ban_user_id: input.ban_user_id.to_owned(),
			ban_reason: input.ban_reason.to_owned(),
			ban_time: input.ban_time.to_owned(),
			user_id: input.user_id.to_owned(),
			release_time,
		};

		self.db_user
			.create_ban_history(&db_input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update_ban_history(&self, input: &UpdateBanHistoryInput) -> Result<(), RepoError> {
		self.db_user
			.update_ban_history(input)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}

	async fn update_user_ban_history_status_to_resolved(
		&self,
		user_id: &str,
	) -> Result<(), RepoError> {
		self.db_user
			.update_user_ban_history_status_to_resolved(user_id)
			.await
			.into_report()
			.change_context(RepoError::Generic)
	}
}

#[derive(Debug, Clone)]
pub struct CreateBanHistoryInput {
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: Option<i32>,
	pub user_id: String,
}
