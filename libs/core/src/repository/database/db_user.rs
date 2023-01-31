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
	async fn count(&self, filter: &UserFilter) -> Result<u64, DbErr>;
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
