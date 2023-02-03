use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "category")]
pub struct Model {
	#[sea_orm(primary_key, column_name = "category_id")]
	pub id: String,
	pub name: String,
	pub slug: String,
	#[sea_orm(nullable)]
	pub description: Option<String>,
	pub level: u32,
	pub postable: bool,
	pub sort_index: i32,
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
			Relation::ParentCategory => Entity::belongs_to(Entity).into(),
			Relation::User => Entity::belongs_to(super::user::Entity).into(),
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

pub type Category = Model;
