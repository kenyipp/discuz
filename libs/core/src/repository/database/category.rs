use crate::constants::UNCLASSIFIED_CATEGORY_ID;
use chrono;
use sea_orm::{sea_query::Expr, DatabaseConnection, *};
use std::sync::Arc;
use uuid::Uuid;

pub use super::entities::category::Category;
use super::entities::{category, post};

#[derive(Debug, Clone)]
pub struct DbCategory {
	db_connection: Arc<DatabaseConnection>,
}

impl DbCategory {
	pub fn new(db_connection: &Arc<DatabaseConnection>) -> DbCategory {
		DbCategory {
			db_connection: db_connection.clone(),
		}
	}
}

#[async_trait]
pub trait DbCategoryTrait {
	async fn list(&self, input: &InputCategoryList) -> Result<Vec<Category>, DbErr>;
	async fn find_by_id(&self, id: &str) -> Result<Option<Category>, DbErr>;
	async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, DbErr>;
	async fn create(&self, input: &CreateCategoryInput) -> Result<String, DbErr>;
	async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, DbErr>;
	async fn delete(&self, id: &str) -> Result<(), DbErr>;
	async fn count(&self, filter: &CategoryFilter) -> Result<u64, DbErr>;
}

#[async_trait]
impl DbCategoryTrait for DbCategory {
	async fn list(&self, input: &InputCategoryList) -> Result<Vec<Category>, DbErr> {
		let mut builder = category::Entity::find().order_by_desc(category::Column::UpdatedAt);

		filter_query_results(&mut builder, &input.filter);

		builder.all(&*self.db_connection).await
	}

	async fn find_by_id(&self, id: &str) -> Result<Option<Category>, DbErr> {
		category::Entity::find()
			.filter(category::Column::Id.eq(id))
			.one(&*self.db_connection)
			.await
	}

	async fn find_by_slug(&self, slug: &str) -> Result<Option<Category>, DbErr> {
		category::Entity::find()
			.filter(category::Column::Slug.eq(slug))
			.one(&*self.db_connection)
			.await
	}

	async fn create(&self, input: &CreateCategoryInput) -> Result<String, DbErr> {
		let category_id = Uuid::new_v4().to_string();
		let now = chrono::offset::Utc::now();
		let category = category::ActiveModel {
			id: Set(category_id.clone()),
			name: Set(input.name.to_owned()),
			slug: Set(input.slug.to_owned()),
			parent_id: Set(input.parent_id.to_owned()),
			count: Set(0),
			description: Set(input.description.to_owned()),
			postable: Set(input.postable.to_owned()),
			level: Set(input.level.to_owned()),
			user_id: Set(input.user_id.to_owned()),
			status_id: Set("A".to_owned()),
			created_at: Set(now),
			updated_at: Set(now),
		};
		category::Entity::insert(category)
			.exec(&*self.db_connection)
			.await?;
		Ok(category_id)
	}

	async fn update(&self, input: &UpdateCategoryInput) -> Result<Category, DbErr> {
		let mut category: category::ActiveModel = self
			.find_by_id(&input.id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("Invalid post category #{}", input.id)))?
			.into();

		category.name = Set(input.name.to_owned());
		category.slug = Set(input.slug.to_owned());
		category.description = Set(input.description.to_owned());
		category.postable = Set(input.postable.to_owned());
		category.level = Set(input.level.to_owned());
		category.parent_id = Set(input.parent_id.to_owned());
		category.user_id = Set(input.user_id.to_owned());
		category.status_id = Set(input.status_id.to_owned());
		category.updated_at = Set(chrono::offset::Utc::now());

		category.update(&*self.db_connection).await?;

		let category = self.find_by_id(&input.id).await?.ok_or_else(|| {
			DbErr::Custom("Unable to retrieve the post category after created".to_owned())
		})?;

		Ok(category)
	}

	async fn delete(&self, id: &str) -> Result<(), DbErr> {
		let mut category: category::ActiveModel = self
			.find_by_id(id)
			.await?
			.ok_or_else(|| DbErr::Custom(format!("Invalid post category #{id}")))?
			.into();

		if category.status_id.take() == Some("D".to_owned()) {
			return Err(DbErr::Custom(
				"The category has been deleted before".to_owned(),
			));
		}

		Update::many(post::Entity)
			.col_expr(
				post::Column::CategoryId,
				Expr::value(UNCLASSIFIED_CATEGORY_ID),
			)
			.filter(post::Column::CategoryId.eq(id))
			.exec(&*self.db_connection)
			.await?;

		category.status_id = Set("D".to_owned());
		category.updated_at = Set(chrono::offset::Utc::now());
		category.update(&*self.db_connection).await?;

		Ok(())
	}

	async fn count(&self, _filter: &CategoryFilter) -> Result<u64, DbErr> {
		let count = category::Entity::find()
			.filter(category::Column::StatusId.eq("A"))
			.count(&*self.db_connection)
			.await?;
		Ok(count)
	}
}

//

fn filter_query_results(builder: &mut Select<category::Entity>, _filter: &CategoryFilter) {
	let mut builder_clone = builder.clone();
	builder_clone = builder_clone.filter(category::Column::StatusId.eq("A"));
	*builder = builder_clone;
}

#[derive(Debug, Clone, Default)]
pub struct InputCategoryList {
	pub filter: CategoryFilter,
}

#[derive(Default, Debug, Clone)]
pub struct CategoryFilter;

#[derive(Debug, Clone)]
pub struct CreateCategoryInput {
	pub name: String,
	pub slug: String,
	pub postable: bool,
	pub level: i32,
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
	pub postable: bool,
	pub level: i32,
	pub parent_id: Option<String>,
	pub user_id: Option<String>,
	pub status_id: String,
}
