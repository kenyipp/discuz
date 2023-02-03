use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "config")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "config_key")]
	pub key: String,
	#[sea_orm(column_name = "config_value")]
	pub value: String,
	#[sea_orm(nullable)]
	pub user_id: Option<String>,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
	User,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
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

pub type Config = Model;
