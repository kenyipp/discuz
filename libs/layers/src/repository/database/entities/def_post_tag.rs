use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "def_post_tag")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "post_tag_id")]
	pub id: String,
	pub name: String,
	pub slug: String,
	pub description: String,
	pub count: i64,
	pub user_id: String,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type DefPostTag = Model;
