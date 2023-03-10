use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = true, column_name = "post_id")]
	pub id: i32,
	pub title: String,
	pub slug: String,
	pub category_id: String,
	pub comment_count: i32,
	pub max_comment_count: i32,
	pub content: String,
	pub user_id: String,
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
	PostReply,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
			Relation::PostReply => Entity::has_many(super::post_reply::Entity).into(),
		}
	}
}

impl Related<super::post_reply::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PostReply.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

pub type Post = Model;
