use chrono::{self, DateTime, Utc};
use sea_orm::{DatabaseConnection, *};
use std::{string::ToString, sync::Arc};

use crate::service::user::constants::UserStatus;

pub use super::entities::user_ban_history::UserBanHistory;
use super::entities::{user, user_ban_history};

#[derive(Debug, Clone)]
pub struct DbUserBanHistory {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbUserBanHistoryTrait {
	async fn find_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, DbErr>;
	async fn create(&self, input: &CreateBanInput) -> Result<i32, DbErr>;
	async fn update(&self, input: &UpdateBanInput) -> Result<(), DbErr>;
	async fn delete(&self, id: i32) -> Result<(), DbErr>;
	async fn update_status(&self, id: i32, status_id: &str) -> Result<(), DbErr>;
	async fn update_user_status(&self, id: &str, status_id: &UserStatus) -> Result<(), DbErr>;
}

#[async_trait]
impl DbUserBanHistoryTrait for DbUserBanHistory {
	async fn find_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, DbErr> {
		user_ban_history::Entity::find()
			.filter(user_ban_history::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn create(&self, input: &CreateBanInput) -> Result<i32, DbErr> {
		let now = chrono::offset::Utc::now();
		let CreateBanInput {
			ban_user_id,
			ban_reason,
			ban_time,
			release_time,
			user_id,
		} = input;
		let input = user_ban_history::ActiveModel {
			ban_user_id: Set(ban_user_id.to_owned()),
			ban_reason: Set(ban_reason.to_owned()),
			ban_time: Set(ban_time.to_owned()),
			release_time: Set(release_time.to_owned()),
			user_id: Set(user_id.to_owned()),
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
			..Default::default()
		};
		let result = user_ban_history::Entity::insert(input)
			.exec(&*self.db_connection)
			.await?;
		Ok(result.last_insert_id)
	}

	async fn update(&self, input: &UpdateBanInput) -> Result<(), DbErr> {
		let now = chrono::offset::Utc::now();
		let UpdateBanInput {
			id,
			ban_user_id,
			ban_reason,
			ban_time,
			release_time,
			user_id,
		} = input;

		let mut history: user_ban_history::ActiveModel = self
			.find_by_id(id.to_owned())
			.await?
			.ok_or_else(|| DbErr::Custom(format!("History with id #{} not exist", input.id)))?
			.into();

		history.ban_user_id = Set(ban_user_id.to_owned());
		history.ban_reason = Set(ban_reason.to_owned());
		history.ban_time = Set(ban_time.to_owned());
		history.release_time = Set(release_time.to_owned());
		history.user_id = Set(user_id.to_owned());
		history.updated_at = Set(now);

		history.update(&*self.db_connection).await?;
		Ok(())
	}

	async fn delete(&self, id: i32) -> Result<(), DbErr> {
		self.update_status(id, "D").await
	}

	async fn update_status(&self, id: i32, status_id: &str) -> Result<(), DbErr> {
		if status_id != "A" && status_id != "D" {
			return Err(DbErr::Custom("Invalid history status".to_string()));
		}

		let mut history: user_ban_history::ActiveModel = self
			.find_by_id(id.to_owned())
			.await?
			.ok_or_else(|| DbErr::Custom(format!("History with id #{id} not exist")))?
			.into();

		history.status_id = Set(status_id.to_owned());

		history.update(&*self.db_connection).await?;
		Ok(())
	}

	async fn update_user_status(&self, id: &str, status_id: &UserStatus) -> Result<(), DbErr> {
		let mut user: user::ActiveModel = user::Entity::find()
			.filter(user::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("User with id #{} not exist", id)))?
			.into();
		user.status_id = Set(status_id.to_string());
		user.update(&*self.db_connection).await?;
		Ok(())
	}
}

impl DbUserBanHistory {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbUserBanHistory {
		DbUserBanHistory {
			db_connection: db_connection.clone(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct CreateBanInput {
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: Option<i32>,
	pub release_time: Option<DateTime<Utc>>,
	pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct UpdateBanInput {
	pub id: i32,
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: Option<i32>,
	pub release_time: Option<DateTime<Utc>>,
	pub user_id: String,
}
