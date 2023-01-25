use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_ban_history")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub ban_user_id: String,
	pub ban_reason: Option<String>,
	pub ban_time: i32,
	pub release_time: DateTimeUtc,
	pub user_id: String,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
	BanUserId,
	User,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
			Relation::BanUserId => Entity::belongs_to(super::user::Entity).into(),
			Relation::User => Entity::belongs_to(super::user::Entity).into(),
		}
	}
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

pub type UserBanHistory = Model;
