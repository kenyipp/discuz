use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "apps_version")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "apps_version_id")]
	pub id: String,
	pub platform: String,
	pub package_id: String,
	pub current_version: String,
	pub minimal_version: String,
	#[sea_orm(nullable)]
	pub user_id: Option<String>,
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

pub type AppsVersion = Model;
