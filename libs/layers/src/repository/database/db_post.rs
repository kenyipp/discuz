use chrono;
use sea_orm::{DatabaseConnection, *};
use std::sync::Arc;
use uuid::Uuid;

use super::entities::{def_post_tag, post, post_tag};
pub use super::entities::{def_post_tag::DefPostTag, post::Post, post_tag::PostTag};

#[derive(Debug, Clone)]
pub struct DbPost {
	db_connection: Arc<DatabaseConnection>,
}

#[async_trait]
pub trait DbPostTrait {
	// Post
	async fn find_by_id(&self, id: &str) -> Result<Option<Post>, DbErr>;
	async fn create(&self, input: &CreatePostInput) -> Result<String, DbErr>;
	async fn update(&self, input: &UploadPostInput) -> Result<(), DbErr>;
	// Def Post Tag
	async fn find_post_tag_by_id(&self, id: &str) -> Result<Option<DefPostTag>, DbErr>;
	async fn create_post_tag(&self, input: &CreatePostTagInput) -> Result<String, DbErr>;
	async fn update_post_tag(&self, input: &UploadPostTagInput) -> Result<(), DbErr>;
	//
	async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<DefPostTag>, DbErr>;
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
	async fn find_by_id(&self, id: &str) -> Result<Option<Post>, DbErr> {
		post::Entity::find()
			.filter(post::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}
	async fn create(&self, input: &CreatePostInput) -> Result<String, DbErr> {
		let post_id = Uuid::new_v4().to_string();
		Ok(post_id)
	}
	async fn update(&self, input: &UploadPostInput) -> Result<(), DbErr> {
		todo!()
	}
	// Post Tag
	async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<DefPostTag>, DbErr> {
		todo!()
	}
	// Def Post Tag
	async fn find_post_tag_by_id(&self, id: &str) -> Result<Option<DefPostTag>, DbErr> {
		def_post_tag::Entity::find()
			.filter(def_post_tag::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}
	async fn create_post_tag(&self, input: &CreatePostTagInput) -> Result<String, DbErr> {
		let post_tag_id = Uuid::new_v4().to_string();
		Ok(post_tag_id)
	}
	async fn update_post_tag(&self, input: &UploadPostTagInput) -> Result<(), DbErr> {
		todo!()
	}
}

#[derive(Debug, Clone)]
pub struct CreatePostInput {
	pub title: String,
	pub slug: String,
	pub post_category_id: String,
	pub content: String,
	pub excerpt: String,
	pub user_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UploadPostInput {
	pub id: String,
	pub title: String,
	pub slug: String,
	pub post_category_id: String,
	pub content: String,
	pub excerpt: String,
	pub user_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreatePostTagInput {
	pub name: String,
	pub slug: String,
	pub description: String,
	pub user_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UploadPostTagInput {
	pub id: String,
	pub name: String,
	pub slug: String,
	pub description: String,
	pub user_id: Option<String>,
}
