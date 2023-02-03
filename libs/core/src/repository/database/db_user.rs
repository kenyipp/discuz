use std::sync::Arc;

use chrono::{self, DateTime, Utc};
use sea_orm::{sea_query::Expr, DatabaseConnection, *};
use uuid::Uuid;

use crate::service::{auth::constants::UserRole, user::constants::UserStatus};

use super::entities::{user, user_ban_history};
pub use super::entities::{user::User, user_ban_history::UserBanHistory};

#[derive(Debug, Clone)]
pub struct DbUser {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbUserTrait {
	// Basic actions
	//  -> Queries
	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, DbErr>;
	async fn count(&self, filter: &UserFilter) -> Result<u64, DbErr>;
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, DbErr>;
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, DbErr>;
	//  -> Mutations
	async fn create(&self, input: &CreateUserInput) -> Result<String, DbErr>;
	async fn update(&self, input: &UpdateUserInput) -> Result<(), DbErr>;
	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), DbErr>;
	async fn update_user_status(&self, id: &str, status_id: &UserStatus) -> Result<(), DbErr>;

	// Ban user
	// -> Queries
	async fn find_ban_history_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, DbErr>;
	// -> Mutations
	async fn create_ban_history(&self, input: &CreateBanHistoryInput) -> Result<i32, DbErr>;
	async fn update_ban_history(&self, input: &UpdateBanHistoryInput) -> Result<(), DbErr>;
	async fn update_user_ban_history_status_to_resolved(&self, user_id: &str) -> Result<(), DbErr>;
}

impl DbUser {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbUser {
		DbUser {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
impl DbUserTrait for DbUser {
	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, DbErr> {
		let mut builder = user::Entity::find()
			.order_by_desc(user::Column::CreatedAt)
			.filter(user::Column::StatusId.eq("normal"))
			.limit(input.limit)
			.offset(input.limit * input.offset);

		filter_query_results(&mut builder, &input.filter);

		builder.all(&*self.db_connection).await
	}

	async fn count(&self, filter: &UserFilter) -> Result<u64, DbErr> {
		let mut builder = user::Entity::find();
		filter_query_results(&mut builder, filter);
		builder.count(&*self.db_connection).await
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<User>, DbErr> {
		user::Entity::find()
			.filter(user::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, DbErr> {
		user::Entity::find()
			.filter(user::Column::Sub.eq(sub))
			.one(&*self.db_connection)
			.await
	}

	async fn create(&self, input: &CreateUserInput) -> Result<String, DbErr> {
		let user_id = Uuid::new_v4().to_string();
		let now = chrono::offset::Utc::now();
		let user = user::ActiveModel {
			id: Set(user_id.clone()),
			sub: Set(input.sub.to_owned()),
			role: Set("user".to_owned()),
			name: Set(input.name.to_owned()),
			email: Set(input.email.to_owned()),
			avatar_url: Set(input.avatar_url.to_owned()),
			status_id: Set("normal".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
			..Default::default()
		};
		user::Entity::insert(user)
			.exec(&*self.db_connection)
			.await?;
		Ok(user_id)
	}

	async fn update(&self, input: &UpdateUserInput) -> Result<(), DbErr> {
		let mut user: user::ActiveModel = self
			.find_by_id(&input.id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("User with id #{} not exist", input.id)))?
			.into();

		user.name = Set(input.name.to_owned());
		user.avatar_url = Set(input.avatar_url.to_owned());
		user.updated_at = Set(chrono::offset::Utc::now());

		user.update(&*self.db_connection).await?;
		Ok(())
	}

	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), DbErr> {
		let mut user: user::ActiveModel = self
			.find_by_id(id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("User with id #{id} not exist")))?
			.into();

		user.role = Set(role.to_string());
		user.update(&*self.db_connection).await?;
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

	async fn find_ban_history_by_id(&self, id: i32) -> Result<Option<UserBanHistory>, DbErr> {
		user_ban_history::Entity::find()
			.filter(user_ban_history::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn create_ban_history(&self, input: &CreateBanHistoryInput) -> Result<i32, DbErr> {
		let now = chrono::offset::Utc::now();
		let input = user_ban_history::ActiveModel {
			ban_user_id: Set(input.ban_user_id.to_owned()),
			ban_reason: Set(input.ban_reason.to_owned()),
			ban_time: Set(input.ban_time.to_owned()),
			release_time: Set(input.release_time.to_owned()),
			user_id: Set(input.user_id.to_owned()),
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

	async fn update_ban_history(&self, input: &UpdateBanHistoryInput) -> Result<(), DbErr> {
		let now = chrono::offset::Utc::now();
		let mut history: user_ban_history::ActiveModel = self
			.find_ban_history_by_id(input.id.to_owned())
			.await?
			.ok_or_else(|| DbErr::Custom(format!("History with id #{} not exist", input.id)))?
			.into();

		history.ban_user_id = Set(input.ban_user_id.to_owned());
		history.ban_reason = Set(input.ban_reason.to_owned());
		history.ban_time = Set(input.ban_time.to_owned());
		history.release_time = Set(input.release_time.to_owned());
		history.user_id = Set(input.user_id.to_owned());
		history.updated_at = Set(now);

		history.update(&*self.db_connection).await?;
		Ok(())
	}

	async fn update_user_ban_history_status_to_resolved(&self, user_id: &str) -> Result<(), DbErr> {
		user_ban_history::Entity::update_many()
			.col_expr(
				user_ban_history::Column::StatusId,
				Expr::value("D".to_owned()),
			)
			.filter(user_ban_history::Column::UserId.eq(user_id))
			.exec(&*self.db_connection)
			.await?;
		Ok(())
	}
}

// Helper functions

fn filter_query_results(builder: &mut Select<user::Entity>, filter: &UserFilter) {
	let mut builder_clone = builder.clone();

	builder_clone = builder_clone.filter(user::Column::StatusId.eq("normal"));

	if let Some(id) = filter.id.to_owned() {
		builder_clone = builder_clone.filter(user::Column::Id.eq(id));
	}
	if let Some(email) = filter.email.to_owned() {
		builder_clone = builder_clone.filter(user::Column::Email.eq(email));
	}
	*builder = builder_clone;
}

//

pub struct InputUserList {
	pub offset: u64,
	pub limit: u64,
	pub filter: UserFilter,
}

impl Default for InputUserList {
	fn default() -> Self {
		InputUserList {
			offset: 0,
			limit: 10,
			filter: UserFilter::default(),
		}
	}
}

#[derive(Default)]
pub struct UserFilter {
	pub id: Option<String>,
	pub email: Option<String>,
}

//

#[derive(Debug, Clone)]
pub struct CreateUserInput {
	pub name: String,
	pub email: String,
	pub sub: String,
	pub avatar_url: Option<String>,
}

//

#[derive(Debug, Clone)]
pub struct UpdateUserInput {
	pub id: String,
	pub name: String,
	pub avatar_url: Option<String>,
}

//

#[derive(Debug, Clone)]
pub struct CreateBanHistoryInput {
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: Option<i32>,
	pub release_time: Option<DateTime<Utc>>,
	pub user_id: String,
}

#[derive(Debug, Clone)]
pub struct UpdateBanHistoryInput {
	pub id: i32,
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: Option<i32>,
	pub release_time: Option<DateTime<Utc>>,
	pub user_id: String,
}
