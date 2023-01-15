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
	async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<PostTag>, DbErr>;
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
		let now = chrono::offset::Utc::now();

		let post = post::ActiveModel {
			id: Set(post_id.clone()),
			title: Set(input.title.clone()),
			slug: Set(input.slug.clone()),
			post_category_id: Set(input.post_category_id.clone()),
			content: Set(input.content.clone()),
			excerpt: Set(input.excerpt.clone()),
			user_id: Set(input.user_id.clone()),
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
		};

		post::Entity::insert(post)
			.exec(&*self.db_connection)
			.await?;
		Ok(post_id)
	}

	async fn update(&self, input: &UploadPostInput) -> Result<(), DbErr> {
		let mut post: post::ActiveModel = self
			.find_by_id(&input.id)
			.await?
			.ok_or(DbErr::Custom(format!(
				"Post with id #{} not exist",
				input.id
			)))?
			.into();

		post.title = Set(input.title.clone());
		post.slug = Set(input.slug.clone());
		post.post_category_id = Set(input.post_category_id.clone());
		post.content = Set(input.content.clone());
		post.excerpt = Set(input.excerpt.clone());
		post.user_id = Set(input.user_id.clone());
		post.updated_at = Set(chrono::offset::Utc::now());

		post.update(&*self.db_connection).await?;
		Ok(())
	}

	// Post Tag
	async fn find_post_tags_by_post_id(&self, id: &str) -> Result<Vec<PostTag>, DbErr> {
		let post_tags: Vec<PostTag> = post_tag::Entity::find()
			.filter(post_tag::Column::PostId.eq(id))
			.all(&*self.db_connection)
			.await?;
		Ok(post_tags)
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
		let now = chrono::offset::Utc::now();

		let post_tag = def_post_tag::ActiveModel {
			id: Set(post_tag_id.clone()),
			name: Set(input.name.clone()),
			slug: Set(input.slug.clone()),
			description: Set(input.description.clone()),
			count: Set(0),
			user_id: Set(input.user_id.clone()),
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
		};

		def_post_tag::Entity::insert(post_tag)
			.exec(&*self.db_connection)
			.await?;
		Ok(post_tag_id)
	}

	async fn update_post_tag(&self, input: &UploadPostTagInput) -> Result<(), DbErr> {
		let mut post_tag: def_post_tag::ActiveModel = self
			.find_post_tag_by_id(&input.id)
			.await?
			.ok_or(DbErr::Custom(format!(
				"Post tag with id #{} not exist",
				input.id
			)))?
			.into();

		post_tag.name = Set(input.name.to_owned());
		post_tag.slug = Set(input.slug.to_owned());
		post_tag.description = Set(input.description.to_owned());
		post_tag.user_id = Set(input.user_id.to_owned());

		post_tag.update(&*self.db_connection).await?;
		Ok(())
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
