use std::{error::Error, fmt};

use async_trait::async_trait;
use error_stack::{IntoReport, Result, ResultExt};

pub use crate::repository::database::db_user::{
	CreateUserInput, DbUser, DbUserTrait, InputUserList, UpdateUserInput, User,
};

#[derive(Debug, Clone)]
pub struct RepoUser {
	db_user: DbUser,
}

#[async_trait]
pub trait RepoUserTrait {
	async fn create(&self, input: &CreateUserInput) -> Result<String, RepoError>;
	async fn update(&self, input: &UpdateUserInput) -> Result<(), RepoError>;
	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, RepoError>;
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepoError>;
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, RepoError>;
}

impl RepoUser {
	pub fn new(db_user: DbUser) -> RepoUser {
		RepoUser { db_user }
	}
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

	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, RepoError> {
		self.db_user
			.list(input)
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

#[derive(Debug)]
pub enum RepoError {
	Generic,
}

impl fmt::Display for RepoError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.write_str("Repo Error")
	}
}

impl Error for RepoError {}
