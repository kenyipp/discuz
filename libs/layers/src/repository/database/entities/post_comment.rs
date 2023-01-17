use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post_comment")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "post_comment_id")]
	pub id: i32,
	pub quote_comment_id: Option<i32>,
	pub post_id: i32,
	pub like_count: i32,
	pub dislike_count: i32,
	pub low_quality: bool,
	pub user_id: String,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type PostComment = Model;
