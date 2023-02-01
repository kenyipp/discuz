use crate::constants::MAX_POST_REPLY_COUNT;
use chrono;
use sea_orm::{sea_query::SimpleExpr, DatabaseConnection, *};
use std::sync::Arc;
// use uuid::Uuid;

pub use super::entities::post::Post;
use super::entities::{def_post_category, post};

#[derive(Debug, Clone)]
pub struct DbPost {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbPostTrait {
	// Post
	async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DbErr>;
	async fn create(&self, input: &CreatePostInput) -> Result<i32, DbErr>;
	async fn update(&self, input: &UpdatePostInput) -> Result<(), DbErr>;
	async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

impl DbPost {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbPost {
		DbPost {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
impl DbPostTrait for DbPost {
	// Post
	async fn find_by_id(&self, id: i32) -> Result<Option<Post>, DbErr> {
		post::Entity::find()
			.filter(post::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn create(&self, input: &CreatePostInput) -> Result<i32, DbErr> {
		let now = chrono::offset::Utc::now();

		let post = post::ActiveModel {
			title: Set(input.title.clone()),
			slug: Set(input.slug.clone()),
			post_category_id: Set(input.post_category_id.clone()),
			content: Set(input.content.clone()),
			user_id: Set(input.user_id.clone()),
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
			..Default::default()
		};

		// Increase the category's post count by 1
		Update::many(def_post_category::Entity)
			.col_expr(
				def_post_category::Column::Count,
				SimpleExpr::Custom(format!(
					"{} + 1",
					def_post_category::Column::Count.to_string()
				)),
			)
			.filter(def_post_category::Column::Id.eq(input.post_category_id.to_owned()))
			.exec(&*self.db_connection)
			.await?;

		let result = post::Entity::insert(post)
			.exec(&*self.db_connection)
			.await?;

		Ok(result.last_insert_id)
	}

	async fn update(&self, input: &UpdatePostInput) -> Result<(), DbErr> {
		let mut post: post::ActiveModel = self
			.find_by_id(input.id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("Post with id #{} not exist", input.id)))?
			.into();

		post.title = Set(input.title.clone());
		post.slug = Set(input.slug.clone());
		post.post_category_id = Set(input.post_category_id.clone());
		post.content = Set(input.content.clone());
		post.max_comment_count = Set(input.max_comment_count.unwrap_or_else(|| {
			post.max_comment_count
				.take()
				.unwrap_or(MAX_POST_REPLY_COUNT)
		}));
		post.user_id = Set(input.user_id.clone());
		post.status_id = Set(input.status_id.clone());
		post.updated_at = Set(chrono::offset::Utc::now());

		post.update(&*self.db_connection).await?;
		Ok(())
	}

	async fn delete(&self, id: i32) -> Result<(), DbErr> {
		let mut post: post::ActiveModel = self
			.find_by_id(id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("Invalid post #{id}")))?
			.into();

		if post.status_id.take() == Some("D".to_owned()) {
			return Err(DbErr::Custom("The post has been deleted before".to_owned()));
		}

		// Decrease the category's post count by 1
		Update::many(def_post_category::Entity)
			.col_expr(
				def_post_category::Column::Count,
				SimpleExpr::Custom(format!(
					"{} - 1",
					def_post_category::Column::Count.to_string()
				)),
			)
			.filter(def_post_category::Column::Id.eq(post.post_category_id.take()))
			.exec(&*self.db_connection)
			.await?;

		post.status_id = Set("D".to_owned());
		post.updated_at = Set(chrono::offset::Utc::now());
		post.update(&*self.db_connection).await?;

		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct CreatePostInput {
	pub title: String,
	pub slug: String,
	pub post_category_id: String,
	pub content: String,
	pub user_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdatePostInput {
	pub id: i32,
	pub title: String,
	pub slug: String,
	pub post_category_id: String,
	pub max_comment_count: Option<i32>,
	pub content: String,
	pub user_id: Option<String>,
	pub status_id: String,
}
