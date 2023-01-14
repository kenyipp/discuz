use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "post_id")]
	pub id: String,
	pub title: String,
	pub slug: String,
	pub post_category_id: String,
	pub content: String,
	pub excerpt: String,
	pub user_id: Option<String>,
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Post = Model;
