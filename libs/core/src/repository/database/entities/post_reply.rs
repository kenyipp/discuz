use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post_reply")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "post_reply_id")]
	pub id: i32,
	pub no_of_reply: u32,
	pub quote_reply_id: Option<i32>,
	pub content: String,
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
pub enum Relation {
	#[sea_orm(
		belongs_to = "Entity",
		from = "Column::QuoteReplyId",
		to = "Column::Id"
	)]
	QuoteReply,
	#[sea_orm(
		belongs_to = "super::post::Entity",
		from = "Column::PostId",
		to = "super::post::Column::Id"
	)]
	Post,
	#[sea_orm(
		belongs_to = "super::user::Entity",
		from = "Column::UserId",
		to = "super::user::Column::Id"
	)]
	User,
}

impl Related<Entity> for Entity {
	fn to() -> RelationDef {
		Relation::QuoteReply.def()
	}
}

impl Related<super::post::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Post.def()
	}
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

pub type PostReply = Model;
