use crate::constants::UNCLASSIFIED_CATEGORY_ID;
use chrono;
use sea_orm::{sea_query::Expr, DatabaseConnection, *};
use std::sync::Arc;
use uuid::Uuid;

pub use super::entities::def_post_category::DefPostCategory;
use super::entities::{def_post_category, post};

#[derive(Debug, Clone)]
pub struct DbPostCategory {
	db_connection: Arc<DatabaseConnection>,
}

impl DbPostCategory {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbPostCategory {
		DbPostCategory {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
pub trait DbPostCategoryTrait {
	async fn list(&self) -> Result<Vec<DefPostCategory>, DbErr>;
	async fn find_by_id(&self, id: &str) -> Result<Option<DefPostCategory>, DbErr>;
	async fn find_by_slug(&self, slug: &str) -> Result<Option<DefPostCategory>, DbErr>;
	async fn create(&self, input: &CreateCategoryInput) -> Result<String, DbErr>;
	async fn update(&self, input: &UpdateCategoryInput) -> Result<DefPostCategory, DbErr>;
	async fn delete(&self, id: &str) -> Result<(), DbErr>;
	async fn count(&self) -> Result<u64, DbErr>;
}

#[async_trait]
impl DbPostCategoryTrait for DbPostCategory {
	async fn list(&self) -> Result<Vec<DefPostCategory>, DbErr> {
		def_post_category::Entity::find()
			.filter(def_post_category::Column::StatusId.eq("A"))
			.all(&*self.db_connection)
			.await
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<DefPostCategory>, DbErr> {
		def_post_category::Entity::find()
			.filter(def_post_category::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn find_by_slug(&self, slug: &str) -> Result<Option<DefPostCategory>, DbErr> {
		def_post_category::Entity::find()
			.filter(def_post_category::Column::Slug.eq(slug))
			.one(&*self.db_connection)
			.await
	}

	async fn create(&self, input: &CreateCategoryInput) -> Result<String, DbErr> {
		let category_id = Uuid::new_v4().to_string();
		let now = chrono::offset::Utc::now();
		let category = def_post_category::ActiveModel {
			id: Set(category_id.clone()),
			name: Set(input.name.to_owned()),
			slug: Set(input.slug.to_owned()),
			parent_id: Set(input.parent_id.to_owned()),
			count: Set(0),
			description: Set(input.description.to_owned()),
			user_id: Set(input.user_id.to_owned()),
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
		};
		def_post_category::Entity::insert(category)
			.exec(&*self.db_connection)
			.await?;
		Ok(category_id)
	}

	async fn update(&self, input: &UpdateCategoryInput) -> Result<DefPostCategory, DbErr> {
		let mut post_category: def_post_category::ActiveModel = self
			.find_by_id(&input.id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("Invalid post category #{}", input.id)))?
			.into();

		post_category.name = Set(input.name.to_owned());
		post_category.slug = Set(input.slug.to_owned());
		post_category.description = Set(input.description.to_owned());
		post_category.parent_id = Set(input.parent_id.to_owned());
		post_category.user_id = Set(input.user_id.to_owned());
		post_category.status_id = Set(input.status_id.to_owned());
		post_category.updated_at = Set(chrono::offset::Utc::now());

		post_category.update(&*self.db_connection).await?;

		let post_category = self.find_by_id(&input.id).await?.ok_or_else(|| {
			DbErr::Custom("Unable to retrieve the post category after created".to_owned())
		})?;

		Ok(post_category)
	}

	async fn delete(&self, id: &str) -> Result<(), DbErr> {
		let mut post_category: def_post_category::ActiveModel = self
			.find_by_id(id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("Invalid post category #{}", id)))?
			.into();

		if post_category.status_id.take() == Some("D".to_owned()) {
			return Err(DbErr::Custom(
				"The post category has been deleted before".to_owned(),
			));
		}

		Update::many(post::Entity)
			.col_expr(
				post::Column::PostCategoryId,
				Expr::value(UNCLASSIFIED_CATEGORY_ID),
			)
			.filter(post::Column::PostCategoryId.eq(id))
			.exec(&*self.db_connection)
			.await?;

		post_category.status_id = Set("D".to_owned());
		post_category.updated_at = Set(chrono::offset::Utc::now());
		post_category.update(&*self.db_connection).await?;

		Ok(())
	}

	async fn count(&self) -> Result<u64, DbErr> {
		let count = def_post_category::Entity::find()
			.filter(def_post_category::Column::StatusId.eq("A"))
			.count(&*self.db_connection)
			.await?;
		Ok(count)
	}
}

#[derive(Debug, Clone)]
pub struct CreateCategoryInput {
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub parent_id: Option<String>,
	pub user_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateCategoryInput {
	pub id: String,
	pub name: String,
	pub slug: String,
	pub description: Option<String>,
	pub parent_id: Option<String>,
	pub user_id: Option<String>,
	pub status_id: String,
}
