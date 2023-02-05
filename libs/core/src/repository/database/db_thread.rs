use chrono::{DateTime, Utc};
use sea_orm::{DatabaseConnection, *};
use std::sync::Arc;

use super::entities::{post, post_reply, user};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct DbThread {
	db_connection: Arc<DatabaseConnection>,
}

impl DbThread {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbThread {
		DbThread {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
pub trait DbThreadTrait {
	// Post
	async fn get_threads(&self, input: &GetThreadsInput) -> Result<Vec<RawThreadOutput>, DbErr>;
}

#[async_trait]
impl DbThreadTrait for DbThread {
	async fn get_threads(&self, input: &GetThreadsInput) -> Result<Vec<RawThreadOutput>, DbErr> {
		todo!()
	}
}

//

pub struct GetThreadsInput {
	category_id: Option<String>,
	last_id: Option<i32>,
	limit: i32,
	order: String,
}

impl Default for GetThreadsInput {
	fn default() -> Self {
		Self {
			category_id: None,
			last_id: None,
			limit: 60,
			order: "DESC".to_owned(),
		}
	}
}

fn filter_query_results(builder: &mut Select<post::Entity>, input: &GetThreadsInput) {
	let mut builder_clone = builder.clone();
	builder_clone = builder_clone.filter(user::Column::StatusId.eq("normal"));

	*builder = builder_clone;
}

#[derive(Debug, FromQueryResult)]
pub struct RawThreadOutput {
	pub id: i32,
	pub quote_reply_id: Option<i32>,
	pub quote_reply_content: Option<String>,
	pub no_of_reply: u32,
	pub content: String,
	pub like_count: i32,
	pub dislike_count: i32,
	pub low_quality: bool,
	pub user_id: String,
	pub user_role: String,
	pub user_name: String,
	pub user_email: String,
	pub user_avatar_url: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}
