use std::sync::Arc;

use chrono;
use sea_orm::{DatabaseConnection, *};
use uuid::Uuid;

use crate::service::auth::constants::UserRole;

use super::entities::user;
pub use super::entities::user::User;

#[derive(Debug, Clone)]
pub struct DbUser {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbUserTrait {
	async fn create(&self, input: &CreateUserInput) -> Result<String, DbErr>;
	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, DbErr>;
	async fn update(&self, input: &UpdateUserInput) -> Result<(), DbErr>;
	async fn find_by_id(&self, id: &str) -> Result<Option<User>, DbErr>;
	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, DbErr>;
	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), DbErr>;
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
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
			..Default::default()
		};
		user::Entity::insert(user)
			.exec(&*self.db_connection)
			.await?;
		Ok(user_id)
	}

	async fn list(&self, input: &InputUserList) -> Result<Vec<User>, DbErr> {
		let mut builder = user::Entity::find()
			.order_by_desc(user::Column::CreatedAt)
			.filter(user::Column::StatusId.eq("A"))
			.limit(input.limit)
			.offset(input.limit * input.offset);
		if input.filter.id.is_some() {
			builder = builder.filter(user::Column::Id.eq(input.filter.id.to_owned()));
		}
		if input.filter.email.is_some() {
			builder = builder.filter(user::Column::Email.eq(input.filter.email.to_owned()));
		}
		builder.all(&*self.db_connection).await
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

	async fn find_by_id(&self, id: &str) -> Result<Option<User>, DbErr> {
		let user = user::Entity::find()
			.filter(user::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await;
		user
	}

	async fn find_by_sub(&self, sub: &str) -> Result<Option<User>, DbErr> {
		let user = user::Entity::find()
			.filter(user::Column::Sub.eq(sub))
			.one(&*self.db_connection)
			.await;
		user
	}

	async fn update_role(&self, id: &str, role: &UserRole) -> Result<(), DbErr> {
		let mut user: user::ActiveModel = self
			.find_by_id(id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("User with id #{} not exist", id)))?
			.into();

		user.role = Set(role.to_string());
		user.update(&*self.db_connection).await?;
		Ok(())
	}
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
