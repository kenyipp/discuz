use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "user_id")]
	pub id: String,
	#[sea_orm(unique)]
	pub sub: String,
	pub role: String,
	pub name: String,
	#[sea_orm(unique)]
	pub email: String,
	#[sea_orm(nullable)]
	pub avatar_url: Option<String>,
	#[sea_orm(nullable)]
	pub notes: Option<String>,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
	PostCategory,
	File,
	PostReply,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
			Relation::PostCategory => Entity::belongs_to(super::def_post_category::Entity).into(),
			Relation::File => Entity::belongs_to(super::file::Entity).into(),
			Relation::PostReply => Entity::belongs_to(super::post_reply::Entity).into(),
		}
	}
}

impl Related<super::def_post_category::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PostCategory.def()
	}
}

impl Related<super::file::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::File.def()
	}
}

impl Related<super::post_reply::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::PostReply.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

pub type User = Model;
