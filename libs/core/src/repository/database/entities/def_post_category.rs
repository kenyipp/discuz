use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "def_post_category")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "post_category_id")]
	pub id: String,
	pub name: String,
	pub slug: String,
	#[sea_orm(nullable)]
	pub description: Option<String>,
	pub parent_id: Option<String>,
	#[sea_orm(default_value = 0)]
	pub count: i64,
	#[sea_orm(nullable)]
	pub user_id: Option<String>,
	#[sea_orm(default_value = "A")]
	pub status_id: String,
	pub created_at: DateTimeUtc,
	pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
	ParentCategory,
	User,
}

impl RelationTrait for Relation {
	fn def(&self) -> RelationDef {
		match self {
			Relation::ParentCategory => Entity::has_one(Entity).into(),
			Relation::User => Entity::has_one(super::user::Entity).into(),
		}
	}
}

impl Related<Entity> for Entity {
	fn to() -> RelationDef {
		Relation::ParentCategory.def()
	}
}

impl Related<super::user::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::User.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}

pub type DefPostCategory = Model;
