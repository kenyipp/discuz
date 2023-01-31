pub use crate::repository::{
	database::db_user::{
		CreateUserInput, DbUser, DbUserTrait, InputUserList, UpdateUserInput, User, UserFilter,
	},
	errors::RepoError,
};
use crate::service::auth::constants::UserRole;
use error_stack::{IntoReport, Result, ResultExt};

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
	async fn create(&self, input: &CreateUserInput) -> Result<String, RepoError>;
	async fn update(&self, input: &UpdateUserInput) -> Result<(), RepoError>;
	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), RepoError>;
	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, RepoError>;
	async fn count(&self, filter: &UserFilter) -> Result<u64, RepoError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepoError>;
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, RepoError>;
}

#[async_trait]
impl RepoUserTrait for RepoUser {
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
}
